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
}

/// Editor mode (Vim-style)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
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
                    "{} Exercise passed! Press 'n' for next.{}",
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
                self.output = ":w save | :c compile | :h hint | :n next | :p prev | :s solution | :auto toggle | :watch toggle | :reset | :q quit".to_string();
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
            render_editor(frame, main, state, true);
        }
        ViewMode::WithSolution => {
            let (editor_area, solution_area) = layout::split_editors_layout(main);
            render_editor(frame, editor_area, state, true);
            render_solution(frame, solution_area, state);
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

            if is_cursor_line && state.mode == EditorMode::Insert {
                // Show cursor in insert mode
                let mut spans = Vec::new();
                let col = state.editor.cursor_col.min(line.len());

                if col > 0 {
                    spans.push(Span::styled(
                        &line[..col],
                        Style::default().fg(theme::colors::TEXT),
                    ));
                }
                spans.push(Span::styled(
                    "█",
                    Style::default().fg(theme::colors::SUCCESS),
                ));
                if col < line.len() {
                    spans.push(Span::styled(
                        &line[col..],
                        Style::default().fg(theme::colors::TEXT),
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
    };

    let keybindings = if state.mode == EditorMode::Command {
        "Enter: run │ Esc: cancel"
    } else {
        "i: edit │ :c compile │ :h hint │ s: solution │ n/p: nav │ :help │ q: quit"
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
    }
}

fn handle_normal_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
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
        KeyCode::Char('n') => {
            state.next_exercise()?;
            Ok(None)
        }
        KeyCode::Char('p') => {
            state.prev_exercise()?;
            Ok(None)
        }
        KeyCode::Char('q') => Ok(Some(true)),
        // Editor navigation - only when NO modifiers (to avoid stealing Ctrl+Up/Down)
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
        // Scroll output with Shift+J/K, Ctrl+Down/Up, or PageUp/PageDown
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
            // Scroll to bottom (large number, will be clamped by display)
            state.output_scroll = u16::MAX;
            Ok(None)
        }
        // Also handle Ctrl+Down/Up for scrolling
        KeyCode::Down if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
            state.output_scroll = state.output_scroll.saturating_add(1);
            Ok(None)
        }
        KeyCode::Up if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
            state.output_scroll = state.output_scroll.saturating_sub(1);
            Ok(None)
        }
        KeyCode::Char('x') => {
            state.modified = true;
            state.editor.delete();
            Ok(None)
        }
        _ => Ok(None),
    }
}

fn handle_insert_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match key.code {
        KeyCode::Esc => {
            state.mode = EditorMode::Normal;
            Ok(None)
        }
        KeyCode::Char(c) => {
            state.modified = true;
            state.editor.insert_char(c);
            Ok(None)
        }
        KeyCode::Enter => {
            state.modified = true;
            state.editor.insert_newline();
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
