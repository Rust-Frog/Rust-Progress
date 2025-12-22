//! Text object commands (dd, yy, daw, diw, caw, ciw, gg, r<char>)

use crate::ui::state::{EditorMode, TuiState};
use anyhow::Result;

/// Handle pending key sequences for text objects and multi-key commands
pub fn handle_pending_keys(key_char: char, state: &mut TuiState) -> Result<Option<bool>> {
    let pending = state.pending_keys.clone();

    match (pending.as_slice(), key_char) {
        // dd - delete line
        (&['d'], 'd') => {
            state.pending_keys.clear();
            state.modified = true;
            state.editor.save_snapshot();
            if let Some(line) = state.editor.delete_line() {
                state.yank_buffer = Some(line);
            }
            Ok(None)
        }
        // yy - yank line
        (&['y'], 'y') => {
            state.pending_keys.clear();
            if let Some(line) = state.editor.get_current_line() {
                state.yank_buffer = Some(line.clone());
            }
            Ok(None)
        }
        // r<char> - replace char
        (&['r'], c) => {
            state.pending_keys.clear();
            state.modified = true;
            state.editor.save_snapshot();
            state.editor.replace_char(c);
            Ok(None)
        }
        // d + a/i or c + a/i - start text object sequence
        (&['d'], 'a') | (&['d'], 'i') | (&['c'], 'a') | (&['c'], 'i') => {
            state.pending_keys.push(key_char);
            Ok(None)
        }
        // daw - delete around word
        (&['d', 'a'], 'w') => {
            state.pending_keys.clear();
            state.modified = true;
            state.editor.save_snapshot();
            if let Some(deleted) = state.editor.delete_around_word() {
                state.yank_buffer = Some(deleted);
            }
            Ok(None)
        }
        // diw - delete inner word
        (&['d', 'i'], 'w') => {
            state.pending_keys.clear();
            state.modified = true;
            state.editor.save_snapshot();
            if let Some(deleted) = state.editor.delete_inner_word() {
                state.yank_buffer = Some(deleted);
            }
            Ok(None)
        }
        // caw - change around word
        (&['c', 'a'], 'w') => {
            state.pending_keys.clear();
            state.modified = true;
            state.editor.save_snapshot();
            if let Some(deleted) = state.editor.delete_around_word() {
                state.yank_buffer = Some(deleted);
            }
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        // ciw - change inner word
        (&['c', 'i'], 'w') => {
            state.pending_keys.clear();
            state.modified = true;
            state.editor.save_snapshot();
            if let Some(deleted) = state.editor.delete_inner_word() {
                state.yank_buffer = Some(deleted);
            }
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        // gg - go to first line
        (['g'], 'g') => {
            state.pending_keys.clear();
            state.editor.goto_first_line();
            Ok(None)
        }
        _ => {
            state.pending_keys.clear();
            Ok(None)
        }
    }
}
