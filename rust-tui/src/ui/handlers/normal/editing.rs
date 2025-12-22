//! Editing commands for normal mode (i,a,o,O,x,p)

use anyhow::Result;
use crossterm::event::KeyCode;

use crate::ui::state::{EditorMode, TuiState};

/// Handle editing operations that modify text or switch modes
pub fn handle_editing(code: KeyCode, state: &mut TuiState) -> Result<Option<bool>> {
    match code {
        KeyCode::Char('i') => {
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char('A') => {
            state.editor.move_to_line_end();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char('o') => {
            state.modified = true;
            state.editor.open_line_below();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char('O') => {
            state.modified = true;
            state.editor.open_line_above();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char('x') => {
            state.modified = true;
            state.editor.delete();
            Ok(None)
        }
        KeyCode::Char('p') => {
            if let Some(text) = &state.yank_buffer {
                state.modified = true;
                if text.contains('\n') || text.ends_with('\n') {
                    state.editor.insert_line_below(text.trim_end().to_string());
                } else {
                    for c in text.chars() {
                        state.editor.insert_char(c);
                    }
                }
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}
