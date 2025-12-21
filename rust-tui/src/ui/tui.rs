//! Main TUI application for Rustlings
//!
//! Custom editor using pure ratatui - no external dependencies with ugly defaults.

use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::style::{Color, Modifier, Style};
use ratatui::{prelude::*, widgets::*};
use std::{
    fs,
    io::stdout,
    time::{Duration, Instant, SystemTime},
};

use crate::{
    app_state::AppState,
    exercise::{OUTPUT_CAPACITY, RunnableExercise},
    ui::{editor::TextEditor, handlers, layout, render, theme},
};

/// Polling interval for file changes (in milliseconds)
const FILE_WATCH_POLL_MS: u64 = 500;

/// View mode for the TUI
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    EditorOnly,
    WithSolution,
    ExpandedOutput,
    HelpModal,
}

/// Editor mode (Vim-style)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
    Visual,
}

/// Main TUI state
pub struct TuiState<'a> {
    pub app_state: &'a mut AppState,
    pub editor: TextEditor,
    pub solution_content: Option<String>,
    pub output: String,
    /// Buffer for exercise runner output (with ANSI codes)
    pub output_buffer: Vec<u8>,
    pub mode: EditorMode,
    pub view_mode: ViewMode,
    pub command_buffer: String,
    pub modified: bool,
    pub file_path: String,
    pub output_scroll: u16,
    pub start_time: Instant,
    pub auto_advance: bool,
    /// Track file modification time for external change detection
    pub last_file_modified: Option<SystemTime>,
    /// Auto-compile on external file changes
    pub auto_compile_on_change: bool,
    /// Vim yank buffer for copy/paste
    pub yank_buffer: Option<String>,
    /// Pending keys for multi-key commands (dd, yy, daw, ciw, etc.)
    pub pending_keys: Vec<char>,
    /// Visual mode selection start position
    pub visual_start_row: usize,
    pub visual_start_col: usize,
    /// Frog learning panel visibility
    pub show_frog: bool,
    /// Current step in Frog content (0-indexed)
    /// Current step in Frog content (0-indexed)
    pub frog_step: usize,
    /// Loaded Frog content steps
    pub current_frog_steps: Vec<String>,
    /// Vertical scroll offset within current Frog slide
    pub frog_scroll: usize,
    /// Total content height (lines) for current frog slide (set by renderer)
    pub frog_content_height: usize,
    /// Visible height for frog content area (set by renderer)
    pub frog_visible_height: usize,
}

impl<'a> TuiState<'a> {
    pub fn new(app_state: &'a mut AppState) -> Result<Self> {
        let exercise = app_state.current_exercise();
        let file_path = exercise.path.to_string();
        let exercise_name = exercise.name.to_string();
        let content = fs::read_to_string(&file_path)?;
        let editor = TextEditor::new(&content);
        let last_file_modified = Self::get_file_modified_time(&file_path);
        let frog_steps = Self::load_frog_content(&file_path, &exercise_name);

        Ok(Self {
            app_state,
            editor,
            solution_content: None,
            output: String::new(),
            output_buffer: Vec::with_capacity(OUTPUT_CAPACITY),
            mode: EditorMode::Normal,
            view_mode: ViewMode::EditorOnly,
            command_buffer: String::new(),
            modified: false,
            file_path,
            output_scroll: 0,
            start_time: Instant::now(),
            auto_advance: true,
            last_file_modified,
            auto_compile_on_change: true,
            yank_buffer: None,
            pending_keys: Vec::new(),
            visual_start_row: 0,
            visual_start_col: 0,
            show_frog: true,
            frog_step: 0,
            current_frog_steps: frog_steps,
            frog_scroll: 0,
            frog_content_height: 0,
            frog_visible_height: 0,
        })
    }

    /// Load Frog content for an exercise from markdown file
    fn load_frog_content(exercise_path: &str, exercise_name: &str) -> Vec<String> {
        // Extract the directory from the exercise path (e.g., "exercises/00_intro/intro1.rs" -> "00_intro")
        let dir = std::path::Path::new(exercise_path)
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("");

        // Try multiple paths to ensure it works whether running from root or rust-tui dir
        let possible_paths = [
            format!("rust-tui/frog/{}/{}.md", dir, exercise_name), // From repo root
            format!("frog/{}/{}.md", dir, exercise_name),          // From rust-tui dir
        ];

        for path in possible_paths {
            if let Ok(content) = fs::read_to_string(&path) {
                return content
                    .split("--- slide ---")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }

        Vec::new()
    }

    /// Get the modification time of a file
    pub fn get_file_modified_time(path: &str) -> Option<SystemTime> {
        fs::metadata(path).ok()?.modified().ok()
    }

    /// Check if file was modified externally and handle it
    pub fn check_external_file_change(&mut self) -> Result<bool> {
        let current_modified = Self::get_file_modified_time(&self.file_path);

        // Check if file was modified externally
        if let (Some(last), Some(current)) = (self.last_file_modified, current_modified) {
            if current > last && !self.modified {
                // File was modified externally, reload it
                self.last_file_modified = Some(current);
                let content = fs::read_to_string(&self.file_path)?;
                self.editor = TextEditor::new(&content);
                self.output = format!("{} File changed externally, reloaded!", theme::icons::INFO);

                // Auto-compile if enabled
                if self.auto_compile_on_change {
                    self.compile()?;
                }
                return Ok(true);
            }
        }
        self.last_file_modified = current_modified;
        Ok(false)
    }

    pub fn save(&mut self) -> Result<()> {
        let content = self.editor.content();
        fs::write(&self.file_path, &content)?;
        self.modified = false;
        // Update the modification time to prevent false "external change" detection
        self.last_file_modified = Self::get_file_modified_time(&self.file_path);
        self.output = format!("{} File saved!", theme::icons::DONE);
        Ok(())
    }

    pub fn compile(&mut self) -> Result<bool> {
        self.save()?;
        self.output = format!(
            "{} Checking {}...",
            theme::icons::COMPILING,
            self.file_path.split('/').last().unwrap_or("file")
        );
        self.output_scroll = 0;

        // Use the proper rustlings exercise runner (handles build, test, clippy)
        let success = self
            .app_state
            .current_exercise()
            .run_exercise(Some(&mut self.output_buffer), self.app_state.cmd_runner())?;

        if success {
            // Mark as done and handle progression properly
            let current_ind = self.app_state.current_exercise_ind();
            self.app_state.set_status(current_ind, true)?;
            self.app_state.write()?;

            // Show success message with solution link if available
            let solution_msg = if let Ok(Some(sol_path)) = self.app_state.current_solution_path() {
                format!("\n\nSolution available: {}", sol_path)
            } else {
                String::new()
            };

            if self.auto_advance {
                // Find next pending exercise (not just next index)
                if let Some(next_pending) = self.find_next_pending_exercise() {
                    self.app_state.set_current_exercise_ind(next_pending)?;
                    self.reload_exercise()?;
                    self.output = format!(
                        "{} Complete! Auto-advanced to: {}",
                        theme::icons::DONE,
                        self.file_path.split('/').last().unwrap_or("next")
                    );
                } else {
                    // All done!
                    self.output = format!(
                        "{} Congratulations! All exercises complete! 🎉",
                        theme::icons::DONE
                    );
                }
            } else {
                self.output = format!(
                    "{} Exercise passed! Press ']' for next.{}",
                    theme::icons::DONE,
                    solution_msg
                );
            }
            Ok(true)
        } else {
            // Show the exercise runner output (includes build errors, test failures, clippy warnings)
            self.output = String::from_utf8_lossy(&self.output_buffer).to_string();
            Ok(false)
        }
    }

    /// Find the next pending exercise index, wrapping around if needed
    pub fn find_next_pending_exercise(&self) -> Option<usize> {
        let exercises = self.app_state.exercises();
        let current = self.app_state.current_exercise_ind();
        let len = exercises.len();

        // First, look after current position
        for i in (current + 1)..len {
            if !exercises[i].done {
                return Some(i);
            }
        }
        // Then wrap around and look from beginning
        for i in 0..current {
            if !exercises[i].done {
                return Some(i);
            }
        }
        None
    }

    pub fn toggle_solution(&mut self) {
        if self.view_mode == ViewMode::WithSolution {
            self.view_mode = ViewMode::EditorOnly;
            self.solution_content = None;
        } else {
            // Use the proper solution path from app_state (handles embedded/community exercises)
            match self.app_state.current_solution_path() {
                Ok(Some(solution_path)) => {
                    if let Ok(content) = fs::read_to_string(&solution_path) {
                        self.solution_content = Some(content);
                        self.view_mode = ViewMode::WithSolution;
                        self.output = format!(
                            "{} Solution loaded: {}",
                            theme::icons::SOLUTION,
                            solution_path
                        );
                    } else {
                        self.output =
                            format!("{} Could not read solution file", theme::icons::ERROR);
                    }
                }
                Ok(None) => {
                    self.output = format!(
                        "{} No solution available for this exercise",
                        theme::icons::ERROR
                    );
                }
                Err(e) => {
                    self.output = format!("{} Error loading solution: {}", theme::icons::ERROR, e);
                }
            }
        }
    }

    pub fn next_exercise(&mut self) -> Result<()> {
        if self.app_state.current_exercise_ind() < self.app_state.exercises().len() - 1 {
            self.app_state
                .set_current_exercise_ind(self.app_state.current_exercise_ind() + 1)?;
            self.reload_exercise()?;
        }
        Ok(())
    }

    pub fn prev_exercise(&mut self) -> Result<()> {
        if self.app_state.current_exercise_ind() > 0 {
            self.app_state
                .set_current_exercise_ind(self.app_state.current_exercise_ind() - 1)?;
            self.reload_exercise()?;
        }
        Ok(())
    }

    pub fn reload_exercise(&mut self) -> Result<()> {
        let exercise = self.app_state.current_exercise();
        self.file_path = exercise.path.to_string();
        let content = fs::read_to_string(&self.file_path)?;
        self.editor = TextEditor::new(&content);
        self.modified = false;
        self.solution_content = None;
        self.view_mode = ViewMode::EditorOnly;
        self.last_file_modified = Self::get_file_modified_time(&self.file_path);
        self.output_scroll = 0;
        self.frog_step = 0;
        self.frog_scroll = 0;
        self.current_frog_steps = Self::load_frog_content(exercise.path, exercise.name);
        Ok(())
    }

    pub fn execute_command(&mut self, cmd: &str) -> Result<Option<bool>> {
        match cmd.trim() {
            "w" => {
                self.save()?;
                Ok(Some(false))
            }
            "q" => {
                if self.modified {
                    self.output =
                        format!("{} Unsaved changes! Use :q! or :wq", theme::icons::ERROR);
                    Ok(Some(false))
                } else {
                    Ok(Some(true))
                }
            }
            "q!" => Ok(Some(true)),
            "wq" | "x" => {
                self.save()?;
                Ok(Some(true))
            }
            "c" => {
                self.compile()?;
                Ok(Some(false))
            }
            "h" | "hint" => {
                let hint = &self.app_state.current_exercise().hint;
                self.output = format!("{} {}", theme::icons::HINT, hint);
                Ok(Some(false))
            }
            "s" | "sol" | "solution" => {
                self.toggle_solution();
                Ok(Some(false))
            }
            "n" | "next" => {
                self.next_exercise()?;
                Ok(Some(false))
            }
            "p" | "prev" => {
                self.prev_exercise()?;
                Ok(Some(false))
            }
            "auto" => {
                self.auto_advance = !self.auto_advance;
                let status = if self.auto_advance { "ON" } else { "OFF" };
                self.output = format!("{} Auto-advance: {}", theme::icons::DONE, status);
                Ok(Some(false))
            }
            "watch" => {
                self.auto_compile_on_change = !self.auto_compile_on_change;
                let status = if self.auto_compile_on_change {
                    "ON"
                } else {
                    "OFF"
                };
                self.output = format!(
                    "{} Auto-compile on file change: {}",
                    theme::icons::DONE,
                    status
                );
                Ok(Some(false))
            }
            "r" | "reload" => {
                self.reload_exercise()?;
                self.output = format!("{} Exercise reloaded from disk", theme::icons::INFO);
                Ok(Some(false))
            }
            "reset" => {
                self.app_state.reset_current_exercise()?;
                self.reload_exercise()?;
                self.output = format!("{} Exercise reset to original", theme::icons::DONE);
                Ok(Some(false))
            }
            "help" => {
                self.view_mode = ViewMode::HelpModal;
                Ok(Some(false))
            }
            _ => {
                self.output = format!(
                    "{} Unknown command: {} (try :help)",
                    theme::icons::ERROR,
                    cmd
                );
                Ok(Some(false))
            }
        }
    }

    pub fn next_frog_step(&mut self) {
        // Check if content overflows and we haven't scrolled to bottom
        let max_scroll = if self.frog_content_height > self.frog_visible_height {
            self.frog_content_height
                .saturating_sub(self.frog_visible_height)
        } else {
            0
        };

        // If content overflows and not at bottom, scroll down instead
        if max_scroll > 0 && self.frog_scroll < max_scroll {
            self.frog_scroll = max_scroll; // Jump to bottom
            return;
        }

        // Only advance if at bottom (or no overflow)
        if self.frog_step < self.current_frog_steps.len().saturating_sub(1) {
            self.frog_step += 1;
            self.frog_scroll = 0; // Reset scroll when changing slides
        }
    }

    pub fn prev_frog_step(&mut self) {
        if self.frog_step > 0 {
            self.frog_step -= 1;
            self.frog_scroll = 0; // Reset scroll when changing slides
        }
    }

    pub fn scroll_frog_up(&mut self) {
        if self.frog_scroll > 0 {
            self.frog_scroll -= 1;
        }
    }

    pub fn scroll_frog_down(&mut self) {
        // Scroll down (we'll cap it in the renderer based on content height)
        self.frog_scroll += 1;
    }
}

/// Render the TUI
fn render(frame: &mut Frame, state: &mut TuiState) {
    let (header, main, footer) = layout::main_layout(frame.area());

    render::render_header(frame, header, state);

    match state.view_mode {
        ViewMode::EditorOnly => {
            // Frog is default panel on the right (if enabled)
            if state.show_frog {
                let (editor_area, right_panel) = layout::split_editors_layout(main);
                render::render_editor(frame, editor_area, state, true);
                render::render_frog_panel(frame, right_panel, state);
            } else {
                // Full width editor when Frog is off
                render::render_editor(frame, main, state, true);
            }
        }
        ViewMode::WithSolution => {
            // Solution replaces Frog when toggled
            let (editor_area, right_panel) = layout::split_editors_layout(main);
            render::render_editor(frame, editor_area, state, true);
            render::render_solution(frame, right_panel, state);
        }
        ViewMode::ExpandedOutput => {
            // In expanded mode, output takes the main area, editor is minimized
            render::render_expanded_output(frame, main, state);
        }
        ViewMode::HelpModal => {
            // Render editor underneath, then overlay the help modal (in main area only)
            if state.show_frog {
                let (editor_area, right_panel) = layout::split_editors_layout(main);
                render::render_editor(frame, editor_area, state, true);
                render::render_frog_panel(frame, right_panel, state);
            } else {
                render::render_editor(frame, main, state, true);
            }
            render::render_help_modal(frame, main);
        }
    }

    render::render_footer(frame, footer, state);
}

fn handle_key(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match state.mode {
        EditorMode::Normal => handlers::handle_normal_mode(key, state),
        EditorMode::Insert => handlers::handle_insert_mode(key, state),
        EditorMode::Command => handlers::handle_command_mode(key, state),
        EditorMode::Visual => handlers::handle_visual_mode(key, state),
    }
}

/// Run the TUI
pub fn run_tui(app_state: &mut AppState) -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = TuiState::new(app_state)?;

    loop {
        terminal.draw(|frame| render(frame, &mut state))?;

        // Poll for events with timeout to allow checking for file changes
        if event::poll(Duration::from_millis(FILE_WATCH_POLL_MS))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        if let Some(should_quit) = handle_key(key, &mut state)? {
                            if should_quit {
                                break;
                            }
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    // Handle mouse scroll for output panel
                    match mouse.kind {
                        event::MouseEventKind::ScrollDown => {
                            state.output_scroll = state.output_scroll.saturating_add(2);
                        }
                        event::MouseEventKind::ScrollUp => {
                            state.output_scroll = state.output_scroll.saturating_sub(2);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            // No event, check for external file changes
            state.check_external_file_change()?;
        }
    }

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show
    )?;
    Ok(())
}
