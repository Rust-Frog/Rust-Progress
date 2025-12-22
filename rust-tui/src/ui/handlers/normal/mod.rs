//! Normal mode key handler - main dispatcher

mod editing;
mod navigation;
mod text_objects;

use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyModifiers};

use crate::ui::state::{EditorMode, TuiState, ViewMode};

/// Handle all normal mode key events
pub fn handle_normal_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    // Dismiss help modal on any key
    if state.view_mode == ViewMode::HelpModal {
        state.view_mode = ViewMode::EditorOnly;
        return Ok(None);
    }

    // Handle pending key sequences first
    if !state.pending_keys.is_empty() {
        if let KeyCode::Char(c) = key.code {
            return text_objects::handle_pending_keys(c, state);
        } else {
            state.pending_keys.clear();
        }
    }

    // Check for modifier keys first
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return handle_ctrl_keys(key.code, state);
    }

    if key.modifiers.contains(KeyModifiers::SHIFT) {
        return handle_shift_keys(key.code, state);
    }

    // Try navigation first
    if matches!(
        key.code,
        KeyCode::Char('h')
            | KeyCode::Char('j')
            | KeyCode::Char('k')
            | KeyCode::Char('l')
            | KeyCode::Char('w')
            | KeyCode::Char('b')
            | KeyCode::Char('0')
            | KeyCode::Char('$')
            | KeyCode::Char('G')
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Up
            | KeyCode::Down
            | KeyCode::Home
            | KeyCode::End
    ) {
        return navigation::handle_navigation(key.code, state);
    }

    // Try editing commands
    if matches!(
        key.code,
        KeyCode::Char('i')
            | KeyCode::Char('A')
            | KeyCode::Char('o')
            | KeyCode::Char('O')
            | KeyCode::Char('x')
            | KeyCode::Char('p')
    ) {
        return editing::handle_editing(key.code, state);
    }

    // Handle remaining commands
    match key.code {
        KeyCode::Char(':') => {
            state.mode = EditorMode::Command;
            state.command_buffer.clear();
            Ok(None)
        }
        KeyCode::Char('v') => {
            state.mode = EditorMode::Visual;
            state.visual_start_row = state.editor.cursor_row;
            state.visual_start_col = state.editor.cursor_col;
            Ok(None)
        }
        KeyCode::Char('s') => {
            state.toggle_solution();
            Ok(None)
        }
        KeyCode::Char('g') => {
            state.pending_keys.push('g');
            Ok(None)
        }
        KeyCode::Char('d') => {
            state.pending_keys.push('d');
            Ok(None)
        }
        KeyCode::Char('y') => {
            state.pending_keys.push('y');
            Ok(None)
        }
        KeyCode::Char('r') => {
            state.pending_keys.push('r');
            Ok(None)
        }
        KeyCode::Char('c') => {
            state.pending_keys.push('c');
            Ok(None)
        }
        KeyCode::Char('u') => Ok(None), // Undo placeholder
        KeyCode::Char(']') => {
            state.next_exercise()?;
            Ok(None)
        }
        KeyCode::Char('[') => {
            state.prev_exercise()?;
            Ok(None)
        }
        KeyCode::Char('q') => Ok(Some(true)),
        KeyCode::PageDown => {
            state.output_scroll = state.output_scroll.saturating_add(10);
            Ok(None)
        }
        KeyCode::PageUp => {
            state.output_scroll = state.output_scroll.saturating_sub(10);
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Handle Ctrl+key combinations
fn handle_ctrl_keys(code: KeyCode, state: &mut TuiState) -> Result<Option<bool>> {
    match code {
        KeyCode::Char('o') => {
            state.view_mode = if state.view_mode == ViewMode::ExpandedOutput {
                ViewMode::EditorOnly
            } else {
                ViewMode::ExpandedOutput
            };
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Handle Shift+key combinations
fn handle_shift_keys(code: KeyCode, state: &mut TuiState) -> Result<Option<bool>> {
    match code {
        // Shift+A - append at end of line
        KeyCode::Char('A') => {
            state.editor.move_to_line_end();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        // Shift+G - go to last line
        KeyCode::Char('G') => {
            state.editor.goto_last_line();
            Ok(None)
        }
        // Shift+O - open line above
        KeyCode::Char('O') => {
            state.modified = true;
            state.editor.open_line_above();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char('F') => {
            state.show_frog = !state.show_frog;
            Ok(None)
        }
        KeyCode::Char('J') => {
            state.output_scroll = state.output_scroll.saturating_add(5);
            Ok(None)
        }
        KeyCode::Char('K') => {
            state.output_scroll = state.output_scroll.saturating_sub(5);
            Ok(None)
        }
        KeyCode::Right => {
            if state.show_frog {
                state.next_frog_step();
            }
            Ok(None)
        }
        KeyCode::Left => {
            if state.show_frog {
                state.prev_frog_step();
            }
            Ok(None)
        }
        KeyCode::Down => {
            if state.show_frog {
                state.scroll_frog_down();
            }
            Ok(None)
        }
        KeyCode::Up => {
            if state.show_frog {
                state.scroll_frog_up();
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}
