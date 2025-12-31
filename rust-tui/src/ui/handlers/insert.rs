use crate::ui::state::{EditorMode, TuiState};
use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyModifiers};

// chars that auto-pair when typed
const AUTO_PAIR: &[(char, char)] = &[('(', ')'), ('{', '}'), ('[', ']'), ('"', '"'), ('\'', '\'')];

// chars we skip over if already at cursor
const SKIP_CHARS: &[char] = &[')', '}', ']', '"', '\''];

fn get_closing_pair(c: char) -> Option<char> {
    AUTO_PAIR
        .iter()
        .find(|(open, _)| *open == c)
        .map(|(_, close)| *close)
}

fn get_current_indent(state: &TuiState) -> usize {
    state
        .editor
        .lines
        .get(state.editor.cursor_row)
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .unwrap_or(0)
}

pub fn handle_insert_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    // ctrl+z / ctrl+shift+z for undo/redo
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return handle_ctrl_key(key.code, state);
    }

    match key.code {
        KeyCode::Esc => {
            state.mode = EditorMode::Normal;
            Ok(None)
        }

        KeyCode::Char(c) => handle_char(c, state),
        KeyCode::Tab => handle_tab(state),
        KeyCode::Enter => handle_enter(state),
        KeyCode::Backspace => handle_backspace(state),
        KeyCode::Delete => handle_delete(state),

        // arrow keys
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

fn handle_ctrl_key(code: KeyCode, state: &mut TuiState) -> Result<Option<bool>> {
    match code {
        KeyCode::Char('z') => {
            if state.editor.undo() {
                state.modified = true;
            }
        }
        KeyCode::Char('Z') => {
            if state.editor.redo() {
                state.modified = true;
            }
        }
        _ => {}
    }
    Ok(None)
}

fn handle_char(c: char, state: &mut TuiState) -> Result<Option<bool>> {
    // skip over closing bracket if already there
    if SKIP_CHARS.contains(&c) && state.editor.char_at_cursor() == Some(c) {
        state.editor.move_right();
        return Ok(None);
    }

    state.modified = true;
    state.editor.save_snapshot();

    // auto-pair: insert both open and close, cursor between
    if let Some(close) = get_closing_pair(c) {
        state.editor.insert_char(c);
        state.editor.insert_char(close);
        state.editor.move_left();
    } else {
        state.editor.insert_char(c);
    }

    Ok(None)
}

fn handle_tab(state: &mut TuiState) -> Result<Option<bool>> {
    state.modified = true;
    state.editor.save_snapshot();
    for _ in 0..4 {
        state.editor.insert_char(' ');
    }
    Ok(None)
}

fn handle_enter(state: &mut TuiState) -> Result<Option<bool>> {
    state.modified = true;
    state.editor.save_snapshot();

    let indent = get_current_indent(state);
    state.editor.insert_newline();

    for _ in 0..indent {
        state.editor.insert_char(' ');
    }
    Ok(None)
}

fn handle_backspace(state: &mut TuiState) -> Result<Option<bool>> {
    state.modified = true;
    state.editor.save_snapshot();
    state.editor.backspace();
    Ok(None)
}

fn handle_delete(state: &mut TuiState) -> Result<Option<bool>> {
    state.modified = true;
    state.editor.save_snapshot();
    state.editor.delete();
    Ok(None)
}
