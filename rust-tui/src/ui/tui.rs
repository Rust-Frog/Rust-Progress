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
    ui::{layout, theme},
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

/// Simple text editor state
pub struct TextEditor {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    scroll_offset: usize,
}

impl TextEditor {
    pub fn new(content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Self {
            lines: if lines.is_empty() {
                vec![String::new()]
            } else {
                lines
            },
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0,
        }
    }

    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn move_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.clamp_col();
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_row < self.lines.len().saturating_sub(1) {
            self.cursor_row += 1;
            self.clamp_col();
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.current_line_len();
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_col < self.current_line_len() {
            self.cursor_col += 1;
        } else if self.cursor_row < self.lines.len().saturating_sub(1) {
            self.cursor_row += 1;
            self.cursor_col = 0;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_row < self.lines.len() {
            let line = &mut self.lines[self.cursor_row];
            if self.cursor_col <= line.len() {
                line.insert(self.cursor_col, c);
                self.cursor_col += 1;
            }
        }
    }

    pub fn insert_newline(&mut self) {
        if self.cursor_row < self.lines.len() {
            let line = self.lines[self.cursor_row].clone();
            let (before, after) = line.split_at(self.cursor_col.min(line.len()));
            self.lines[self.cursor_row] = before.to_string();
            self.lines.insert(self.cursor_row + 1, after.to_string());
            self.cursor_row += 1;
            self.cursor_col = 0;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            let line = &mut self.lines[self.cursor_row];
            if self.cursor_col <= line.len() {
                line.remove(self.cursor_col - 1);
                self.cursor_col -= 1;
            }
        } else if self.cursor_row > 0 {
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].len();
            self.lines[self.cursor_row].push_str(&current_line);
        }
    }

    pub fn delete(&mut self) {
        if self.cursor_col < self.current_line_len() {
            self.lines[self.cursor_row].remove(self.cursor_col);
        } else if self.cursor_row < self.lines.len() - 1 {
            let next_line = self.lines.remove(self.cursor_row + 1);
            self.lines[self.cursor_row].push_str(&next_line);
        }
    }

    fn current_line_len(&self) -> usize {
        self.lines
            .get(self.cursor_row)
            .map(|l| l.len())
            .unwrap_or(0)
    }

    fn clamp_col(&mut self) {
        self.cursor_col = self.cursor_col.min(self.current_line_len());
    }

    pub fn update_scroll(&mut self, visible_height: usize) {
        if self.cursor_row < self.scroll_offset {
            self.scroll_offset = self.cursor_row;
        } else if self.cursor_row >= self.scroll_offset + visible_height {
            self.scroll_offset = self.cursor_row - visible_height + 1;
        }
    }

    // === Vim Movement Methods ===

    pub fn move_to_line_start(&mut self) {
        self.cursor_col = 0;
    }

    pub fn move_to_line_end(&mut self) {
        self.cursor_col = self.current_line_len();
    }

    pub fn move_to_first_non_whitespace(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_row) {
            self.cursor_col = line.chars().take_while(|c| c.is_whitespace()).count();
        }
    }

    pub fn move_word_forward(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            let mut col = self.cursor_col;

            // Skip current word (non-whitespace)
            while col < chars.len() && !chars[col].is_whitespace() {
                col += 1;
            }
            // Skip whitespace
            while col < chars.len() && chars[col].is_whitespace() {
                col += 1;
            }

            if col >= chars.len() && self.cursor_row < self.lines.len() - 1 {
                // Move to next line
                self.cursor_row += 1;
                self.cursor_col = 0;
                self.move_to_first_non_whitespace();
            } else {
                self.cursor_col = col;
            }
        }
    }

    pub fn move_word_backward(&mut self) {
        if self.cursor_col == 0 && self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.current_line_len();
        }

        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            let mut col = self.cursor_col.saturating_sub(1);

            // Skip whitespace backwards
            while col > 0 && chars.get(col).map_or(false, |c| c.is_whitespace()) {
                col -= 1;
            }
            // Skip word backwards
            while col > 0 && chars.get(col - 1).map_or(false, |c| !c.is_whitespace()) {
                col -= 1;
            }

            self.cursor_col = col;
        }
    }

    pub fn goto_first_line(&mut self) {
        self.cursor_row = 0;
        self.clamp_col();
    }

    pub fn goto_last_line(&mut self) {
        self.cursor_row = self.lines.len().saturating_sub(1);
        self.clamp_col();
    }

    // === Vim Editing Methods ===

    pub fn delete_line(&mut self) -> Option<String> {
        if self.lines.len() > 1 {
            let deleted = self.lines.remove(self.cursor_row);
            if self.cursor_row >= self.lines.len() {
                self.cursor_row = self.lines.len().saturating_sub(1);
            }
            self.clamp_col();
            Some(deleted)
        } else {
            // Last line - just clear it
            let deleted = std::mem::take(&mut self.lines[0]);
            self.cursor_col = 0;
            Some(deleted)
        }
    }

    pub fn open_line_below(&mut self) {
        self.lines.insert(self.cursor_row + 1, String::new());
        self.cursor_row += 1;
        self.cursor_col = 0;
    }

    pub fn open_line_above(&mut self) {
        self.lines.insert(self.cursor_row, String::new());
        self.cursor_col = 0;
    }

    pub fn get_current_line(&self) -> Option<&String> {
        self.lines.get(self.cursor_row)
    }

    pub fn insert_line_below(&mut self, content: String) {
        self.lines.insert(self.cursor_row + 1, content);
    }

    pub fn insert_line_above(&mut self, content: String) {
        self.lines.insert(self.cursor_row, content);
    }

    pub fn replace_char(&mut self, c: char) {
        if let Some(line) = self.lines.get_mut(self.cursor_row) {
            let mut chars: Vec<char> = line.chars().collect();
            if self.cursor_col < chars.len() {
                chars[self.cursor_col] = c;
                *line = chars.into_iter().collect();
            }
        }
    }

    /// Get the character at the current cursor position (for skip-over logic)
    pub fn char_at_cursor(&self) -> Option<char> {
        self.lines
            .get(self.cursor_row)
            .and_then(|line| line.chars().nth(self.cursor_col))
    }

    /// Delete inner word (diw) - just the word, not surrounding spaces
    pub fn delete_inner_word(&mut self) -> Option<String> {
        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            if chars.is_empty() || self.cursor_col >= chars.len() {
                return None;
            }

            // Find word boundaries
            let mut start = self.cursor_col;
            let mut end = self.cursor_col;

            // Expand left while we're on word chars
            while start > 0 && !chars[start - 1].is_whitespace() {
                start -= 1;
            }
            // Expand right while we're on word chars
            while end < chars.len() && !chars[end].is_whitespace() {
                end += 1;
            }

            // Extract and delete
            let deleted: String = chars[start..end].iter().collect();
            let new_line: String = chars[..start].iter().chain(chars[end..].iter()).collect();
            self.lines[self.cursor_row] = new_line;
            self.cursor_col = start;
            Some(deleted)
        } else {
            None
        }
    }

    /// Delete around word (daw) - word plus trailing/leading whitespace
    pub fn delete_around_word(&mut self) -> Option<String> {
        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            if chars.is_empty() || self.cursor_col >= chars.len() {
                return None;
            }

            // Find word boundaries
            let mut start = self.cursor_col;
            let mut end = self.cursor_col;

            // Expand left while we're on word chars
            while start > 0 && !chars[start - 1].is_whitespace() {
                start -= 1;
            }
            // Expand right while we're on word chars
            while end < chars.len() && !chars[end].is_whitespace() {
                end += 1;
            }

            // Also include trailing whitespace (or leading if at end of line)
            let orig_end = end;
            while end < chars.len() && chars[end].is_whitespace() {
                end += 1;
            }
            // If no trailing whitespace, try leading
            if end == orig_end && start > 0 {
                while start > 0 && chars[start - 1].is_whitespace() {
                    start -= 1;
                }
            }

            // Extract and delete
            let deleted: String = chars[start..end].iter().collect();
            let new_line: String = chars[..start].iter().chain(chars[end..].iter()).collect();
            self.lines[self.cursor_row] = new_line;
            self.cursor_col = start.min(self.lines[self.cursor_row].len());
            Some(deleted)
        } else {
            None
        }
    }
}

/// Main TUI state
pub struct TuiState<'a> {
    app_state: &'a mut AppState,
    editor: TextEditor,
    solution_content: Option<String>,
    output: String,
    /// Buffer for exercise runner output (with ANSI codes)
    output_buffer: Vec<u8>,
    mode: EditorMode,
    view_mode: ViewMode,
    command_buffer: String,
    modified: bool,
    file_path: String,
    output_scroll: u16,
    start_time: Instant,
    auto_advance: bool,
    /// Track file modification time for external change detection
    last_file_modified: Option<SystemTime>,
    /// Auto-compile on external file changes
    auto_compile_on_change: bool,
    /// Vim yank buffer for copy/paste
    yank_buffer: Option<String>,
    /// Pending keys for multi-key commands (dd, yy, daw, ciw, etc.)
    pending_keys: Vec<char>,
    /// Visual mode selection start position
    visual_start_row: usize,
    visual_start_col: usize,
    /// Frog learning panel visibility
    show_frog: bool,
    /// Current step in Frog content (0-indexed)
    frog_step: usize,
}

impl<'a> TuiState<'a> {
    pub fn new(app_state: &'a mut AppState) -> Result<Self> {
        let exercise = app_state.current_exercise();
        let file_path = exercise.path.to_string();
        let content = fs::read_to_string(&file_path)?;
        let editor = TextEditor::new(&content);
        let last_file_modified = Self::get_file_modified_time(&file_path);

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
        })
    }

    /// Get the modification time of a file
    fn get_file_modified_time(path: &str) -> Option<SystemTime> {
        fs::metadata(path).ok()?.modified().ok()
    }

    /// Check if file was modified externally and handle it
    fn check_external_file_change(&mut self) -> Result<bool> {
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

    fn save(&mut self) -> Result<()> {
        let content = self.editor.content();
        fs::write(&self.file_path, &content)?;
        self.modified = false;
        // Update the modification time to prevent false "external change" detection
        self.last_file_modified = Self::get_file_modified_time(&self.file_path);
        self.output = format!("{} File saved!", theme::icons::DONE);
        Ok(())
    }

    fn compile(&mut self) -> Result<bool> {
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
    fn find_next_pending_exercise(&self) -> Option<usize> {
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

    fn toggle_solution(&mut self) {
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

    fn next_exercise(&mut self) -> Result<()> {
        if self.app_state.current_exercise_ind() < self.app_state.exercises().len() - 1 {
            self.app_state
                .set_current_exercise_ind(self.app_state.current_exercise_ind() + 1)?;
            self.reload_exercise()?;
        }
        Ok(())
    }

    fn prev_exercise(&mut self) -> Result<()> {
        if self.app_state.current_exercise_ind() > 0 {
            self.app_state
                .set_current_exercise_ind(self.app_state.current_exercise_ind() - 1)?;
            self.reload_exercise()?;
        }
        Ok(())
    }

    fn reload_exercise(&mut self) -> Result<()> {
        let exercise = self.app_state.current_exercise();
        self.file_path = exercise.path.to_string();
        let content = fs::read_to_string(&self.file_path)?;
        self.editor = TextEditor::new(&content);
        self.modified = false;
        self.solution_content = None;
        self.view_mode = ViewMode::EditorOnly;
        self.last_file_modified = Self::get_file_modified_time(&self.file_path);
        self.output_scroll = 0;
        Ok(())
    }

    fn execute_command(&mut self, cmd: &str) -> Result<Option<bool>> {
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
}

/// Render the TUI
fn render(frame: &mut Frame, state: &mut TuiState) {
    let (header, main, footer) = layout::main_layout(frame.area());

    render_header(frame, header, state);

    match state.view_mode {
        ViewMode::EditorOnly => {
            // Frog is default panel on the right (if enabled)
            if state.show_frog {
                let (editor_area, right_panel) = layout::split_editors_layout(main);
                render_editor(frame, editor_area, state, true);
                render_frog_panel(frame, right_panel, state);
            } else {
                // Full width editor when Frog is off
                render_editor(frame, main, state, true);
            }
        }
        ViewMode::WithSolution => {
            // Solution replaces Frog when toggled
            let (editor_area, right_panel) = layout::split_editors_layout(main);
            render_editor(frame, editor_area, state, true);
            render_solution(frame, right_panel, state);
        }
        ViewMode::ExpandedOutput => {
            // In expanded mode, output takes the main area, editor is minimized
            render_expanded_output(frame, main, state);
        }
        ViewMode::HelpModal => {
            // Render editor underneath, then overlay the help modal (in main area only)
            if state.show_frog {
                let (editor_area, right_panel) = layout::split_editors_layout(main);
                render_editor(frame, editor_area, state, true);
                render_frog_panel(frame, right_panel, state);
            } else {
                render_editor(frame, main, state, true);
            }
            render_help_modal(frame, main);
        }
    }

    render_footer(frame, footer, state);
}

fn render_header(frame: &mut Frame, area: Rect, state: &TuiState) {
    let done = state.app_state.n_done();
    let total = state.app_state.exercises().len();
    let current = state.app_state.current_exercise_ind() + 1;
    let exercise_name = state.file_path.split('/').last().unwrap_or("unknown");
    let modified_indicator = if state.modified { " ●" } else { "" };

    let is_done = state.app_state.exercises()[state.app_state.current_exercise_ind()].done;
    let exercise_style = if is_done {
        Style::default()
            .fg(theme::colors::SUCCESS)
            .add_modifier(Modifier::BOLD | Modifier::CROSSED_OUT)
    } else {
        Style::default()
            .fg(theme::colors::TEXT)
            .add_modifier(Modifier::BOLD)
    };

    let header_line = Line::from(vec![
        Span::styled(
            format!(" {} ", theme::icons::CRAB),
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "RUSTLINGS",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" │ ", Style::default().fg(theme::colors::MUTED)),
        Span::styled(
            format!("{}{}", exercise_name, modified_indicator),
            exercise_style,
        ),
        Span::styled(" │ ", Style::default().fg(theme::colors::MUTED)),
        Span::styled(
            format!("Exercise {}/{}", current, total),
            Style::default().fg(theme::colors::TEXT_DIM),
        ),
        Span::styled(" │ ", Style::default().fg(theme::colors::MUTED)),
        Span::styled(
            format!("{} done", done),
            Style::default()
                .fg(theme::colors::SUCCESS)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let header = Paragraph::new(header_line);
    frame.render_widget(header, area);
}

fn render_editor(frame: &mut Frame, area: Rect, state: &mut TuiState, is_active: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if is_active {
            Style::default().fg(theme::colors::PRIMARY)
        } else {
            Style::default().fg(theme::colors::MUTED)
        })
        .title(Span::styled(
            " Editor ",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let visible_height = inner.height as usize;
    state.editor.update_scroll(visible_height);

    // Line number width
    let line_count = state.editor.lines.len();
    let line_num_width = (line_count.to_string().len() + 2) as u16;

    let editor_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(line_num_width), Constraint::Min(10)])
        .split(inner);

    // Render line numbers
    let visible_lines: Vec<Line> = (state.editor.scroll_offset
        ..state.editor.scroll_offset + visible_height)
        .filter_map(|i| {
            if i < line_count {
                Some(Line::from(Span::styled(
                    format!("{:>width$} ", i + 1, width = line_num_width as usize - 2),
                    Style::default().fg(theme::colors::MUTED),
                )))
            } else {
                None
            }
        })
        .collect();

    let line_nums_widget = Paragraph::new(visible_lines);
    frame.render_widget(line_nums_widget, editor_chunks[0]);

    // Render code with syntax highlighting
    let code_lines: Vec<Line> = state
        .editor
        .lines
        .iter()
        .skip(state.editor.scroll_offset)
        .take(visible_height)
        .enumerate()
        .map(|(i, line)| {
            let actual_row = state.editor.scroll_offset + i;
            let is_cursor_line = actual_row == state.editor.cursor_row;

            // Check if this line is in the visual selection
            let in_visual_mode = state.mode == EditorMode::Visual;
            let visual_bounds = if in_visual_mode {
                Some(get_selection_bounds(state))
            } else {
                None
            };

            // Visual mode rendering with selection highlighting
            if in_visual_mode {
                render_visual_line(line, actual_row, &visual_bounds, state, is_cursor_line)
            } else if is_cursor_line
                && (state.mode == EditorMode::Insert || state.mode == EditorMode::Normal)
            {
                // Use different cursor colors for different modes
                let cursor_color = if state.mode == EditorMode::Insert {
                    theme::colors::SUCCESS // Green for Insert
                } else {
                    theme::colors::PRIMARY // Orange for Normal
                };

                // Use char indices for proper UTF-8 handling
                let chars: Vec<char> = line.chars().collect();
                let col = state.editor.cursor_col.min(chars.len());

                let mut spans = Vec::new();

                // Text before cursor
                if col > 0 {
                    let before: String = chars[..col].iter().collect();
                    spans.push(Span::styled(
                        before,
                        Style::default().fg(theme::colors::TEXT),
                    ));
                }

                // Cursor character (inverted colors)
                if col < chars.len() {
                    spans.push(Span::styled(
                        chars[col].to_string(),
                        Style::default()
                            .fg(theme::colors::BACKGROUND)
                            .bg(cursor_color),
                    ));
                    // Text after cursor
                    if col + 1 < chars.len() {
                        let after: String = chars[col + 1..].iter().collect();
                        spans.push(Span::styled(
                            after,
                            Style::default().fg(theme::colors::TEXT),
                        ));
                    }
                } else {
                    // At end of line, show a block cursor (space with background)
                    spans.push(Span::styled(
                        " ",
                        Style::default()
                            .fg(theme::colors::BACKGROUND)
                            .bg(cursor_color),
                    ));
                }
                Line::from(spans)
            } else {
                // Syntax highlighting
                highlight_rust_line(line, is_cursor_line)
            }
        })
        .collect();

    let code_widget = Paragraph::new(code_lines);
    frame.render_widget(code_widget, editor_chunks[1]);
}

fn highlight_rust_line(line: &str, _is_current: bool) -> Line<'static> {
    // Simple syntax highlighting
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Comments
        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            let comment: String = chars[i..].iter().collect();
            spans.push(Span::styled(
                comment,
                Style::default().fg(theme::colors::COMMENT),
            ));
            break;
        }

        // Strings
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1;
                }
                i += 1;
            }
            if i < chars.len() {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            spans.push(Span::styled(s, Style::default().fg(theme::colors::STRING)));
            continue;
        }

        // Keywords
        let keywords = [
            "fn", "let", "mut", "if", "else", "match", "for", "while", "loop", "return", "use",
            "mod", "pub", "struct", "enum", "impl", "trait", "where", "const", "static", "self",
            "Self", "true", "false", "None", "Some", "Ok", "Err",
        ];
        let mut found_keyword = false;
        for kw in keywords {
            if i + kw.len() <= chars.len() {
                let word: String = chars[i..i + kw.len()].iter().collect();
                if word == kw {
                    let next_char = chars.get(i + kw.len());
                    let prev_char = if i > 0 { chars.get(i - 1) } else { None };
                    if (next_char.is_none() || !next_char.unwrap().is_alphanumeric())
                        && (prev_char.is_none() || !prev_char.unwrap().is_alphanumeric())
                    {
                        spans.push(Span::styled(
                            word,
                            Style::default().fg(theme::colors::KEYWORD),
                        ));
                        i += kw.len();
                        found_keyword = true;
                        break;
                    }
                }
            }
        }
        if found_keyword {
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let num: String = chars[start..i].iter().collect();
            spans.push(Span::styled(
                num,
                Style::default().fg(theme::colors::NUMBER),
            ));
            continue;
        }

        // Default text
        spans.push(Span::styled(
            chars[i].to_string(),
            Style::default().fg(theme::colors::TEXT),
        ));
        i += 1;
    }

    if spans.is_empty() {
        spans.push(Span::styled(" ", Style::default()));
    }

    Line::from(spans)
}

fn render_solution(frame: &mut Frame, area: Rect, state: &TuiState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::INFO))
        .title(Span::styled(
            format!(" {} Solution ", theme::icons::SOLUTION),
            Style::default()
                .fg(theme::colors::INFO)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(ref content) = state.solution_content {
        let lines: Vec<Line> = content
            .lines()
            .map(|line| highlight_rust_line(line, false))
            .collect();
        let solution_widget = Paragraph::new(lines);
        frame.render_widget(solution_widget, inner);
    }
}

/// Render the Frog learning panel with educational content
fn render_frog_panel(frame: &mut Frame, area: Rect, state: &TuiState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::SUCCESS))
        .title(Span::styled(
            " 🐸 Frog ",
            Style::default()
                .fg(theme::colors::SUCCESS)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Get the current exercise name from file path
    let exercise_name = state.file_path.split('/').last().unwrap_or("unknown");

    // Placeholder content - will be loaded from frog/ folder
    let content = vec![
        Line::from(vec![Span::styled(
            format!("Exercise: {}", exercise_name),
            Style::default()
                .fg(theme::colors::TEXT)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Welcome to the Frog learning panel! 🐸",
            Style::default().fg(theme::colors::MUTED),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Content coming soon...",
            Style::default().fg(theme::colors::MUTED),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            format!("Step: {}/1", state.frog_step + 1),
            Style::default().fg(theme::colors::PRIMARY),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Shift+F toggle │ Shift+N/P navigate",
            Style::default().fg(theme::colors::MUTED),
        )]),
    ];

    let frog_widget = Paragraph::new(content).wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(frog_widget, inner);
}

fn render_expanded_output(frame: &mut Frame, area: Rect, state: &TuiState) {
    // Full-screen output view
    let clean_output = strip_ansi_codes(&state.output);
    let output_lines: Vec<&str> = clean_output.lines().collect();
    let total_lines = output_lines.len();
    let visible_height = area.height.saturating_sub(2) as usize;

    let max_scroll = total_lines.saturating_sub(visible_height);
    let scroll_pos = (state.output_scroll as usize).min(max_scroll);

    let output_style = if state.output.contains("✓") || state.output.contains("complete") {
        Style::default().fg(theme::colors::SUCCESS)
    } else if state.output.contains("error") || state.output.contains("✗") {
        Style::default().fg(theme::colors::ERROR)
    } else if state.output.contains("💡") {
        Style::default().fg(theme::colors::ACCENT)
    } else {
        Style::default().fg(theme::colors::TEXT)
    };

    let scroll_indicator = format!(
        " Output [{}/{}] ← Press 'o' to collapse ",
        scroll_pos + visible_height.min(total_lines),
        total_lines
    );

    let output_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::PRIMARY))
        .title(Span::styled(
            scroll_indicator,
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ));

    let output = Paragraph::new(clean_output.as_str())
        .block(output_block)
        .style(output_style)
        .wrap(Wrap { trim: false })
        .scroll((scroll_pos as u16, 0));
    frame.render_widget(output, area);
}

fn render_help_modal(frame: &mut Frame, area: Rect) {
    // Calculate centered modal area (60% width, 70% height)
    let modal_width = (area.width * 60 / 100).max(50).min(80);
    let modal_height = (area.height * 70 / 100).max(20).min(35);
    let modal_x = area.x + (area.width.saturating_sub(modal_width)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_height)) / 2;
    let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

    // Clear the area behind the modal
    frame.render_widget(Clear, modal_area);

    // Build help content
    let help_text = vec![
        Line::from(vec![Span::styled(
            "   Rustlings TUI Help   ",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  NAVIGATION",
            Style::default()
                .fg(theme::colors::INFO)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ]         ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Next exercise"),
        ]),
        Line::from(vec![
            Span::styled("  [         ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Previous exercise"),
        ]),
        Line::from(vec![
            Span::styled("  Shift+J/K ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Scroll output down/up"),
        ]),
        Line::from(vec![
            Span::styled("  PgDn/PgUp ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Fast scroll output"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  EDITING",
            Style::default()
                .fg(theme::colors::INFO)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  i         ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Enter Insert mode"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Return to Normal mode"),
        ]),
        Line::from(vec![
            Span::styled("  h/j/k/l   ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Vim cursor movement"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  COMMANDS",
            Style::default()
                .fg(theme::colors::INFO)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  :w        ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Save file"),
        ]),
        Line::from(vec![
            Span::styled("  :c        ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Compile/check"),
        ]),
        Line::from(vec![
            Span::styled("  :hint / h ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Show hint"),
        ]),
        Line::from(vec![
            Span::styled("  :sol / s  ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle solution view"),
        ]),
        Line::from(vec![
            Span::styled("  Shift+F   ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle 🐸 Frog panel"),
        ]),
        Line::from(vec![
            Span::styled("  o         ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Expand output panel"),
        ]),
        Line::from(vec![
            Span::styled("  :auto     ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle auto-advance"),
        ]),
        Line::from(vec![
            Span::styled("  :watch    ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle auto-compile"),
        ]),
        Line::from(vec![
            Span::styled("  :reset    ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Reset exercise"),
        ]),
        Line::from(vec![
            Span::styled("  :q / q    ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Quit"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Press Esc or any key to close",
            Style::default().fg(theme::colors::MUTED),
        )]),
    ];

    let help_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::PRIMARY))
        .title(Span::styled(
            " 🦀 Help ",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    let help_paragraph = Paragraph::new(help_text)
        .block(help_block)
        .style(Style::default().fg(theme::colors::TEXT));

    frame.render_widget(help_paragraph, modal_area);
}

fn render_footer(frame: &mut Frame, area: Rect, state: &TuiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    // Output panel - strip ANSI codes for clean display
    let clean_output = strip_ansi_codes(&state.output);
    let output_lines: Vec<&str> = clean_output.lines().collect();
    let total_lines = output_lines.len();
    let visible_height = chunks[0].height.saturating_sub(2) as usize; // Account for borders

    // Clamp scroll to valid range
    let max_scroll = total_lines.saturating_sub(visible_height);
    let scroll_pos = (state.output_scroll as usize).min(max_scroll);

    let output_style = if state.output.contains("✓") || state.output.contains("complete") {
        Style::default().fg(theme::colors::SUCCESS)
    } else if state.output.contains("error") || state.output.contains("✗") {
        Style::default().fg(theme::colors::ERROR)
    } else if state.output.contains("💡") {
        Style::default().fg(theme::colors::ACCENT)
    } else {
        Style::default().fg(theme::colors::TEXT)
    };

    // Show scroll position if scrolled
    let scroll_indicator = if scroll_pos > 0 {
        format!(
            " Output [{}/{}] ",
            scroll_pos + visible_height.min(total_lines),
            total_lines
        )
    } else {
        " Output ".to_string()
    };

    let output_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::MUTED))
        .title(Span::styled(
            scroll_indicator,
            Style::default().fg(theme::colors::MUTED),
        ));

    let output = Paragraph::new(clean_output.as_str())
        .block(output_block)
        .style(output_style)
        .wrap(Wrap { trim: false })
        .scroll((scroll_pos as u16, 0));
    frame.render_widget(output, chunks[0]);

    // Progress bar with pulsing orange ball
    let done = state.app_state.n_done() as usize;
    let total = state.app_state.exercises().len();
    let progress_width = chunks[1].width.saturating_sub(10) as usize; // Leave room for percentage

    // Smooth pulsing animation for the ball
    let elapsed_ms = state.start_time.elapsed().as_millis();
    let pulse_phase = (elapsed_ms % 1000) as f32 / 1000.0; // 0.0 to 1.0 over 1 second

    // Sinusoidal pulse for smooth brightness transition
    let brightness = ((pulse_phase * std::f32::consts::PI * 2.0).sin() + 1.0) / 2.0; // 0.0 to 1.0
    let r = 255;
    let g = (80.0 + brightness * 100.0) as u8; // 80-180
    let b = (30.0 + brightness * 80.0) as u8; // 30-110
    let ball_color = Color::Rgb(r, g, b);

    let filled = if total > 0 {
        (done * progress_width) / total
    } else {
        0
    };
    let empty = progress_width.saturating_sub(filled);

    // Progress percentage
    let percent = if total > 0 { (done * 100) / total } else { 0 };
    let percent_str = format!(" {}% ", percent);

    // Build the clean progress line: [orange━━━●gray━━━] XX%
    let mut spans = vec![Span::styled(" ", Style::default())];

    // Filled portion (completed) - orange
    if filled > 0 {
        spans.push(Span::styled(
            "━".repeat(filled),
            Style::default().fg(theme::colors::PRIMARY), // Orange
        ));
    }

    // The pulsing ball
    spans.push(Span::styled(
        "●",
        Style::default().fg(ball_color).add_modifier(Modifier::BOLD),
    ));

    // Empty portion (remaining) - gray
    if empty > 0 {
        spans.push(Span::styled(
            "━".repeat(empty),
            Style::default().fg(theme::colors::MUTED), // Gray
        ));
    }

    // Percentage indicator
    spans.push(Span::styled(
        percent_str,
        Style::default().fg(if percent == 100 {
            theme::colors::SUCCESS
        } else {
            theme::colors::TEXT_DIM
        }),
    ));

    let progress_line = Line::from(spans);
    let progress_bar = Paragraph::new(progress_line);
    frame.render_widget(progress_bar, chunks[1]);

    // Status bar
    let mode_span = match state.mode {
        EditorMode::Normal => Span::styled(" NORMAL ", theme::mode_normal_style()),
        EditorMode::Insert => Span::styled(" INSERT ", theme::mode_insert_style()),
        EditorMode::Command => Span::styled(
            format!(" :{} ", state.command_buffer),
            theme::mode_command_style(),
        ),
        EditorMode::Visual => Span::styled(" VISUAL ", theme::mode_visual_style()),
    };

    let keybindings = if state.mode == EditorMode::Command {
        "Enter: run │ Esc: cancel"
    } else {
        "i: edit │ :c compile │ :h hint │ s: solution │ [/]: nav │ :help │ q: quit"
    };

    let status_line = Line::from(vec![
        mode_span,
        Span::styled(" ", Style::default()),
        Span::styled(keybindings, Style::default().fg(theme::colors::TEXT_DIM)),
    ]);
    let status_bar = Paragraph::new(status_line);
    frame.render_widget(status_bar, chunks[2]);
}

fn handle_key(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match state.mode {
        EditorMode::Normal => handle_normal_mode(key, state),
        EditorMode::Insert => handle_insert_mode(key, state),
        EditorMode::Command => handle_command_mode(key, state),
        EditorMode::Visual => handle_visual_mode(key, state),
    }
}

fn handle_normal_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    // If help modal is open, any key dismisses it
    if state.view_mode == ViewMode::HelpModal {
        state.view_mode = ViewMode::EditorOnly;
        return Ok(None);
    }

    // Handle pending key sequences (dd, yy, r<char>, daw, diw, caw, ciw)
    if !state.pending_keys.is_empty() {
        let pending = state.pending_keys.clone();

        if let KeyCode::Char(c) = key.code {
            match (pending.as_slice(), c) {
                // dd - delete line
                (&['d'], 'd') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(line) = state.editor.delete_line() {
                        state.yank_buffer = Some(line);
                    }
                    return Ok(None);
                }
                // yy - yank line
                (&['y'], 'y') => {
                    state.pending_keys.clear();
                    if let Some(line) = state.editor.get_current_line() {
                        state.yank_buffer = Some(line.clone());
                    }
                    return Ok(None);
                }
                // r<char> - replace char
                (&['r'], c) => {
                    state.pending_keys.clear();
                    state.modified = true;
                    state.editor.replace_char(c);
                    return Ok(None);
                }
                // d + a/i - start text object delete
                (&['d'], 'a') | (&['d'], 'i') | (&['c'], 'a') | (&['c'], 'i') => {
                    state.pending_keys.push(c);
                    return Ok(None);
                }
                // daw - delete around word
                (&['d', 'a'], 'w') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(deleted) = state.editor.delete_around_word() {
                        state.yank_buffer = Some(deleted);
                    }
                    return Ok(None);
                }
                // diw - delete inner word
                (&['d', 'i'], 'w') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(deleted) = state.editor.delete_inner_word() {
                        state.yank_buffer = Some(deleted);
                    }
                    return Ok(None);
                }
                // caw - change around word
                (&['c', 'a'], 'w') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(deleted) = state.editor.delete_around_word() {
                        state.yank_buffer = Some(deleted);
                    }
                    state.mode = EditorMode::Insert;
                    return Ok(None);
                }
                // ciw - change inner word
                (&['c', 'i'], 'w') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(deleted) = state.editor.delete_inner_word() {
                        state.yank_buffer = Some(deleted);
                    }
                    state.mode = EditorMode::Insert;
                    return Ok(None);
                }
                // c alone starts pending
                (&['c'], 'c') => {
                    // cc - change entire line (delete line content, stay on line, insert mode)
                    state.pending_keys.clear();
                    state.modified = true;
                    state.editor.delete_line();
                    state.editor.insert_line_above(String::new());
                    state.mode = EditorMode::Insert;
                    return Ok(None);
                }
                _ => {
                    // Invalid sequence, clear and ignore
                    state.pending_keys.clear();
                    return Ok(None);
                }
            }
        } else {
            // Non-char key while pending, cancel
            state.pending_keys.clear();
            return Ok(None);
        }
    }

    match key.code {
        KeyCode::Char('i') => {
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char(':') => {
            state.mode = EditorMode::Command;
            state.command_buffer.clear();
            Ok(None)
        }
        KeyCode::Char('s') => {
            state.toggle_solution();
            Ok(None)
        }
        // Ctrl+O - Toggle expanded output view (remapped from 'o')
        KeyCode::Char('o') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
            state.view_mode = if state.view_mode == ViewMode::ExpandedOutput {
                ViewMode::EditorOnly
            } else {
                ViewMode::ExpandedOutput
            };
            Ok(None)
        }
        // Shift+F - Toggle Frog learning panel
        KeyCode::Char('F') if key.modifiers.contains(event::KeyModifiers::SHIFT) => {
            state.show_frog = !state.show_frog;
            Ok(None)
        }
        // Shift+N - Next Frog step
        KeyCode::Char('N') if key.modifiers.contains(event::KeyModifiers::SHIFT) => {
            if state.show_frog {
                state.frog_step = state.frog_step.saturating_add(1);
            }
            Ok(None)
        }
        // Shift+P - Previous Frog step
        KeyCode::Char('P') if key.modifiers.contains(event::KeyModifiers::SHIFT) => {
            if state.show_frog {
                state.frog_step = state.frog_step.saturating_sub(1);
            }
            Ok(None)
        }
        // === VIM EDITING ===
        // o - open line below
        KeyCode::Char('o') => {
            state.modified = true;
            state.editor.open_line_below();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        // O - open line above
        KeyCode::Char('O') => {
            state.modified = true;
            state.editor.open_line_above();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        // A - append at end of line
        KeyCode::Char('A') => {
            state.editor.move_to_line_end();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        // I - insert at first non-whitespace
        KeyCode::Char('I') => {
            state.editor.move_to_first_non_whitespace();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        // d - start delete sequence (waiting for second key)
        KeyCode::Char('d') => {
            state.pending_keys.push('d');
            Ok(None)
        }
        // y - start yank sequence
        KeyCode::Char('y') => {
            state.pending_keys.push('y');
            Ok(None)
        }
        // p - paste below
        KeyCode::Char('p') => {
            if let Some(ref content) = state.yank_buffer.clone() {
                state.modified = true;
                state.editor.insert_line_below(content.clone());
                state.editor.move_down();
            }
            Ok(None)
        }
        // P - paste above
        KeyCode::Char('P') => {
            if let Some(ref content) = state.yank_buffer.clone() {
                state.modified = true;
                state.editor.insert_line_above(content.clone());
            }
            Ok(None)
        }
        // x - delete char under cursor
        KeyCode::Char('x') => {
            state.modified = true;
            state.editor.delete();
            Ok(None)
        }
        // r - replace char (next char typed will replace current)
        KeyCode::Char('r') => {
            state.pending_keys.push('r');
            Ok(None)
        }
        // c - start change sequence (for cc, ciw, caw)
        KeyCode::Char('c') => {
            state.pending_keys.push('c');
            Ok(None)
        }
        // === VIM MOVEMENT ===
        // w - word forward
        KeyCode::Char('w') => {
            state.editor.move_word_forward();
            Ok(None)
        }
        // b - word backward
        KeyCode::Char('b') => {
            state.editor.move_word_backward();
            Ok(None)
        }
        // 0 - line start
        KeyCode::Char('0') => {
            state.editor.move_to_line_start();
            Ok(None)
        }
        // $ - line end
        KeyCode::Char('$') => {
            state.editor.move_to_line_end();
            Ok(None)
        }
        // g - go to first line
        KeyCode::Char('g') => {
            state.editor.goto_first_line();
            Ok(None)
        }
        // G - go to last line
        KeyCode::Char('G') => {
            state.editor.goto_last_line();
            Ok(None)
        }
        // Navigation - [ for previous, ] for next exercise
        KeyCode::Char(']') => {
            state.next_exercise()?;
            Ok(None)
        }
        KeyCode::Char('[') => {
            state.prev_exercise()?;
            Ok(None)
        }
        KeyCode::Char('q') => Ok(Some(true)),
        // Editor navigation - only when NO modifiers
        KeyCode::Char('h') | KeyCode::Left if key.modifiers.is_empty() => {
            state.editor.move_left();
            Ok(None)
        }
        KeyCode::Char('j') | KeyCode::Down if key.modifiers.is_empty() => {
            state.editor.move_down();
            Ok(None)
        }
        KeyCode::Char('k') | KeyCode::Up if key.modifiers.is_empty() => {
            state.editor.move_up();
            Ok(None)
        }
        KeyCode::Char('l') | KeyCode::Right if key.modifiers.is_empty() => {
            state.editor.move_right();
            Ok(None)
        }
        // Scroll output
        KeyCode::Char('J') | KeyCode::Down
            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
        {
            state.output_scroll = state.output_scroll.saturating_add(1);
            Ok(None)
        }
        KeyCode::Char('K') | KeyCode::Up
            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
        {
            state.output_scroll = state.output_scroll.saturating_sub(1);
            Ok(None)
        }
        KeyCode::PageDown => {
            state.output_scroll = state.output_scroll.saturating_add(10);
            Ok(None)
        }
        KeyCode::PageUp => {
            state.output_scroll = state.output_scroll.saturating_sub(10);
            Ok(None)
        }
        KeyCode::Home => {
            state.output_scroll = 0;
            Ok(None)
        }
        KeyCode::End => {
            state.output_scroll = u16::MAX;
            Ok(None)
        }
        KeyCode::Down if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
            state.output_scroll = state.output_scroll.saturating_add(1);
            Ok(None)
        }
        KeyCode::Up if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
            state.output_scroll = state.output_scroll.saturating_sub(1);
            Ok(None)
        }
        // v - Enter Visual mode for text selection
        KeyCode::Char('v') => {
            state.mode = EditorMode::Visual;
            state.visual_start_row = state.editor.cursor_row;
            state.visual_start_col = state.editor.cursor_col;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Handle Visual mode (text selection with highlighting)
fn handle_visual_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match key.code {
        // Escape - exit visual mode
        KeyCode::Esc => {
            state.mode = EditorMode::Normal;
            Ok(None)
        }
        // Movement keys - extend selection
        KeyCode::Char('h') | KeyCode::Left => {
            state.editor.move_left();
            Ok(None)
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state.editor.move_down();
            Ok(None)
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.editor.move_up();
            Ok(None)
        }
        KeyCode::Char('l') | KeyCode::Right => {
            state.editor.move_right();
            Ok(None)
        }
        // Word movement
        KeyCode::Char('w') => {
            state.editor.move_word_forward();
            Ok(None)
        }
        KeyCode::Char('b') => {
            state.editor.move_word_backward();
            Ok(None)
        }
        // Line start/end
        KeyCode::Char('0') => {
            state.editor.move_to_line_start();
            Ok(None)
        }
        KeyCode::Char('$') => {
            state.editor.move_to_line_end();
            Ok(None)
        }
        // y - Yank (copy) selection
        KeyCode::Char('y') => {
            let selected_text = get_visual_selection(state);
            state.yank_buffer = Some(selected_text);
            state.mode = EditorMode::Normal;
            Ok(None)
        }
        // d - Delete selection
        KeyCode::Char('d') => {
            delete_visual_selection(state);
            state.mode = EditorMode::Normal;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Get text within the visual selection
fn get_visual_selection(state: &TuiState) -> String {
    let (start_row, start_col, end_row, end_col) = get_selection_bounds(state);

    if start_row == end_row {
        // Single line selection
        if let Some(line) = state.editor.lines.get(start_row) {
            let start = start_col.min(line.len());
            let end = (end_col + 1).min(line.len());
            return line[start..end].to_string();
        }
    } else {
        // Multi-line selection
        let mut result = String::new();
        for row in start_row..=end_row {
            if let Some(line) = state.editor.lines.get(row) {
                if row == start_row {
                    result.push_str(&line[start_col.min(line.len())..]);
                    result.push('\n');
                } else if row == end_row {
                    result.push_str(&line[..(end_col + 1).min(line.len())]);
                } else {
                    result.push_str(line);
                    result.push('\n');
                }
            }
        }
        return result;
    }
    String::new()
}

/// Delete text within the visual selection
fn delete_visual_selection(state: &mut TuiState) {
    let (start_row, start_col, end_row, end_col) = get_selection_bounds(state);

    // Yank before deleting
    let selected = get_visual_selection(state);
    state.yank_buffer = Some(selected);
    state.modified = true;

    if start_row == end_row {
        // Single line deletion
        if let Some(line) = state.editor.lines.get_mut(start_row) {
            let start = start_col.min(line.len());
            let end = (end_col + 1).min(line.len());
            line.replace_range(start..end, "");
        }
    } else {
        // Multi-line deletion: keep start of first line, end of last line, remove middle lines
        let first_part: String = state
            .editor
            .lines
            .get(start_row)
            .map(|l| l[..start_col.min(l.len())].to_string())
            .unwrap_or_default();
        let last_part: String = state
            .editor
            .lines
            .get(end_row)
            .map(|l| l[(end_col + 1).min(l.len())..].to_string())
            .unwrap_or_default();

        // Remove lines from start_row+1 to end_row
        for _ in start_row..end_row {
            if start_row + 1 < state.editor.lines.len() {
                state.editor.lines.remove(start_row + 1);
            }
        }
        // Merge first and last parts
        if let Some(line) = state.editor.lines.get_mut(start_row) {
            *line = first_part + &last_part;
        }
    }

    // Move cursor to start of selection
    state.editor.cursor_row = start_row;
    state.editor.cursor_col = start_col;
}

/// Get normalized selection bounds (start <= end)
fn get_selection_bounds(state: &TuiState) -> (usize, usize, usize, usize) {
    let cur_row = state.editor.cursor_row;
    let cur_col = state.editor.cursor_col;
    let start_row = state.visual_start_row;
    let start_col = state.visual_start_col;

    if start_row < cur_row || (start_row == cur_row && start_col <= cur_col) {
        (start_row, start_col, cur_row, cur_col)
    } else {
        (cur_row, cur_col, start_row, start_col)
    }
}

/// Render a line with visual selection highlighting
fn render_visual_line(
    line: &str,
    row: usize,
    bounds: &Option<(usize, usize, usize, usize)>,
    state: &TuiState,
    is_cursor_line: bool,
) -> Line<'static> {
    let chars: Vec<char> = line.chars().collect();
    let mut spans = Vec::new();

    // Get selection bounds
    let (start_row, start_col, end_row, end_col) = bounds.unwrap_or((0, 0, 0, 0));

    // Check if this row is in the selection range
    if row < start_row || row > end_row {
        // Not in selection, render normally
        return highlight_rust_line(line, is_cursor_line);
    }

    // Determine selection columns for this row
    let (sel_start, sel_end) = if start_row == end_row {
        // Single line selection
        (start_col, end_col + 1)
    } else if row == start_row {
        // First line of multi-line selection
        (start_col, chars.len())
    } else if row == end_row {
        // Last line of multi-line selection
        (0, end_col + 1)
    } else {
        // Middle line - fully selected
        (0, chars.len())
    };

    let sel_start = sel_start.min(chars.len());
    let sel_end = sel_end.min(chars.len());

    // Text before selection
    if sel_start > 0 {
        let before: String = chars[..sel_start].iter().collect();
        spans.push(Span::styled(
            before,
            Style::default().fg(theme::colors::TEXT),
        ));
    }

    // Selected text (highlighted with magenta background)
    if sel_start < sel_end {
        let selected: String = chars[sel_start..sel_end].iter().collect();
        spans.push(Span::styled(
            selected,
            Style::default()
                .fg(theme::colors::BACKGROUND)
                .bg(theme::colors::PRIMARY), // Orange highlight for selection
        ));
    }

    // Cursor at current position in visual mode
    let cursor_col = state.editor.cursor_col;
    if is_cursor_line && cursor_col >= sel_end && cursor_col < chars.len() {
        // Show cursor after selection if visible
        let cursor_char = chars[cursor_col].to_string();
        spans.push(Span::styled(
            cursor_char,
            Style::default()
                .fg(theme::colors::BACKGROUND)
                .bg(theme::colors::SUCCESS), // Green cursor
        ));
        if cursor_col + 1 < chars.len() {
            let after: String = chars[cursor_col + 1..].iter().collect();
            spans.push(Span::styled(
                after,
                Style::default().fg(theme::colors::TEXT),
            ));
        }
    } else if sel_end < chars.len() {
        // Text after selection
        let after: String = chars[sel_end..].iter().collect();
        spans.push(Span::styled(
            after,
            Style::default().fg(theme::colors::TEXT),
        ));
    }

    // Handle empty line with cursor
    if chars.is_empty() && is_cursor_line {
        spans.push(Span::styled(
            " ",
            Style::default()
                .fg(theme::colors::BACKGROUND)
                .bg(theme::colors::SUCCESS),
        ));
    }

    Line::from(spans)
}

fn handle_insert_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match key.code {
        KeyCode::Esc => {
            state.mode = EditorMode::Normal;
            Ok(None)
        }
        KeyCode::Char(c) => {
            // Skip-over: if typing a closing bracket that's already at cursor, just move past it
            let skip_chars = [')', '}', ']', '"', '\''];
            if skip_chars.contains(&c) && state.editor.char_at_cursor() == Some(c) {
                state.editor.move_right();
                return Ok(None);
            }

            state.modified = true;
            // Auto-brackets: insert closing pair and move cursor between
            match c {
                '(' => {
                    state.editor.insert_char('(');
                    state.editor.insert_char(')');
                    state.editor.move_left();
                }
                '{' => {
                    state.editor.insert_char('{');
                    state.editor.insert_char('}');
                    state.editor.move_left();
                }
                '[' => {
                    state.editor.insert_char('[');
                    state.editor.insert_char(']');
                    state.editor.move_left();
                }
                '"' => {
                    state.editor.insert_char('"');
                    state.editor.insert_char('"');
                    state.editor.move_left();
                }
                '\'' => {
                    state.editor.insert_char('\'');
                    state.editor.insert_char('\'');
                    state.editor.move_left();
                }
                _ => {
                    state.editor.insert_char(c);
                }
            }
            Ok(None)
        }
        KeyCode::Tab => {
            state.modified = true;
            for _ in 0..4 {
                state.editor.insert_char(' ');
            }
            Ok(None)
        }
        KeyCode::Enter => {
            state.modified = true;
            // Auto-indentation similar to VS Code:
            // 1. Get current indentation
            let current_indent = if state.editor.cursor_row < state.editor.lines.len() {
                let line = &state.editor.lines[state.editor.cursor_row];
                line.chars().take_while(|c| c.is_whitespace()).count()
            } else {
                0
            };

            state.editor.insert_newline();

            // 2. Apply indentation to new line
            for _ in 0..current_indent {
                state.editor.insert_char(' ');
            }
            Ok(None)
        }
        KeyCode::Backspace => {
            state.modified = true;
            state.editor.backspace();
            Ok(None)
        }
        KeyCode::Delete => {
            state.modified = true;
            state.editor.delete();
            Ok(None)
        }
        KeyCode::Left => {
            state.editor.move_left();
            Ok(None)
        }
        KeyCode::Right => {
            state.editor.move_right();
            Ok(None)
        }
        KeyCode::Up => {
            state.editor.move_up();
            Ok(None)
        }
        KeyCode::Down => {
            state.editor.move_down();
            Ok(None)
        }
        _ => Ok(None),
    }
}

fn handle_command_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match key.code {
        KeyCode::Enter => {
            let cmd = state.command_buffer.clone();
            state.mode = EditorMode::Normal;
            state.execute_command(&cmd)
        }
        KeyCode::Esc => {
            state.mode = EditorMode::Normal;
            Ok(None)
        }
        KeyCode::Backspace => {
            state.command_buffer.pop();
            if state.command_buffer.is_empty() {
                state.mode = EditorMode::Normal;
            }
            Ok(None)
        }
        KeyCode::Char(c) => {
            state.command_buffer.push(c);
            Ok(None)
        }
        _ => Ok(None),
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

fn strip_ansi_codes(s: &str) -> String {
    let mut clean = String::with_capacity(s.len());
    let mut in_escape = false;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            in_escape = true;
            if let Some('[') = chars.peek() {
                chars.next();
            }
            continue;
        }
        if in_escape {
            if c.is_ascii_alphabetic() || c == '@' {
                in_escape = false;
            }
            continue;
        }
        clean.push(c);
    }
    clean
}
