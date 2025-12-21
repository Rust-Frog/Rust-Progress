//! Editor view and key handling
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::{EditorEventHandler, EditorView};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::state::{EditorMode, EditorState};

/// Color scheme
pub mod colors {
    use ratatui::style::Color;
    pub const RUST_ORANGE: Color = Color::Rgb(247, 76, 0);
    pub const SUCCESS_GREEN: Color = Color::Green;
    pub const ERROR_RED: Color = Color::Red;
    pub const HINT_CYAN: Color = Color::Cyan;
}

/// Render the editor UI
pub fn render_editor(frame: &mut Frame, state: &mut EditorState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    render_editor_panel(frame, chunks[0], state);
    render_output_panel(frame, chunks[1], state);
    render_status_bar(frame, state);
}

/// Render the main editor panel
fn render_editor_panel(frame: &mut Frame, area: Rect, state: &mut EditorState) {
    let editor_area = Rect {
        height: area.height.saturating_sub(2),
        ..area
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::RUST_ORANGE))
        .title(format!(
            " {} {} ",
            if state.modified { "●" } else { "" },
            state.file_path.split('/').last().unwrap_or("editor")
        ));

    let inner = block.inner(editor_area);
    frame.render_widget(block, editor_area);

    // Render the edtui editor
    let editor_view = EditorView::new(&mut state.editor);
    frame.render_widget(editor_view, inner);
}

/// Render the output/hints panel
fn render_output_panel(frame: &mut Frame, area: Rect, state: &EditorState) {
    let output_area = Rect {
        height: area.height.saturating_sub(2),
        ..area
    };

    let (title, content, color) = if state.show_hint {
        ("Hint", state.hint.clone(), colors::HINT_CYAN)
    } else if let Some(ref result) = state.compile_result {
        if result.contains("successful") || result.contains("saved") {
            ("Output", result.clone(), colors::SUCCESS_GREEN)
        } else {
            ("Output", result.clone(), colors::ERROR_RED)
        }
    } else {
        (
            "Output",
            "Press :c to compile, h to toggle hints".to_string(),
            Color::Gray,
        )
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(format!(" {} ", title));

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, output_area);
}

/// Render the status bar
fn render_status_bar(frame: &mut Frame, state: &EditorState) {
    let area = frame.area();
    let status_area = Rect {
        x: 0,
        y: area.height.saturating_sub(2),
        width: area.width,
        height: 2,
    };

    let mode_str = match state.mode {
        EditorMode::Normal => " NORMAL ",
        EditorMode::Insert => " INSERT ",
        EditorMode::Command => " COMMAND ",
    };

    let mode_style = match state.mode {
        EditorMode::Normal => Style::default().bg(Color::Blue).fg(Color::White),
        EditorMode::Insert => Style::default().bg(Color::Green).fg(Color::Black),
        EditorMode::Command => Style::default().bg(Color::Yellow).fg(Color::Black),
    };

    let status = if state.mode == EditorMode::Command {
        format!(":{}", state.command_buffer)
    } else {
        format!(
            "{} | :w save | :q quit | :c compile | h hint | Esc exit",
            mode_str
        )
    };

    let paragraph = Paragraph::new(status).style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .patch(mode_style),
    );

    frame.render_widget(paragraph, status_area);
}

/// Actions the editor can take
pub enum EditorAction {
    Quit,
    ExecuteCommand(String),
}

/// Handle key events
pub fn handle_key_event(key: KeyEvent, state: &mut EditorState) -> Option<EditorAction> {
    match state.mode {
        EditorMode::Normal => handle_normal_mode(key, state),
        EditorMode::Insert => handle_insert_mode(key, state),
        EditorMode::Command => handle_command_mode(key, state),
    }
}

fn handle_normal_mode(key: KeyEvent, state: &mut EditorState) -> Option<EditorAction> {
    let mut handler = EditorEventHandler::default();

    match key.code {
        KeyCode::Char('i') => {
            state.enter_insert_mode();
            None
        }
        KeyCode::Char(':') => {
            state.enter_command_mode();
            None
        }
        KeyCode::Char('h') if key.modifiers.is_empty() => {
            state.toggle_hint();
            None
        }
        KeyCode::Esc => Some(EditorAction::Quit),
        _ => {
            // Let edtui handle vim motions
            handler.on_key_event(key, &mut state.editor);
            None
        }
    }
}

fn handle_insert_mode(key: KeyEvent, state: &mut EditorState) -> Option<EditorAction> {
    let mut handler = EditorEventHandler::default();

    match key.code {
        KeyCode::Esc => {
            state.enter_normal_mode();
            None
        }
        KeyCode::Char(_) => {
            state.modified = true;
            // Let edtui handle the character input
            handler.on_key_event(key, &mut state.editor);
            None
        }
        _ => {
            handler.on_key_event(key, &mut state.editor);
            None
        }
    }
}

fn handle_command_mode(key: KeyEvent, state: &mut EditorState) -> Option<EditorAction> {
    match key.code {
        KeyCode::Enter => {
            let cmd = state.command_buffer.clone();
            state.enter_normal_mode();
            Some(EditorAction::ExecuteCommand(cmd))
        }
        KeyCode::Esc => {
            state.enter_normal_mode();
            None
        }
        KeyCode::Backspace => {
            state.command_buffer.pop();
            if state.command_buffer.is_empty() {
                state.enter_normal_mode();
            }
            None
        }
        KeyCode::Char(c) => {
            state.command_buffer.push(c);
            None
        }
        _ => None,
    }
}
