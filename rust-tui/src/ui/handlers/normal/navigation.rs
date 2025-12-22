//! Navigation commands for normal mode (h,j,k,l,w,b,0,$,gg,G)

use anyhow::Result;
use crossterm::event::KeyCode;

use crate::ui::state::TuiState;

/// Handle basic cursor navigation
pub fn handle_navigation(code: KeyCode, state: &mut TuiState) -> Result<Option<bool>> {
    match code {
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
        KeyCode::Char('0') | KeyCode::Home => {
            state.editor.move_to_line_start();
            Ok(None)
        }
        KeyCode::Char('$') | KeyCode::End => {
            state.editor.move_to_line_end();
            Ok(None)
        }
        KeyCode::Char('w') => {
            state.editor.move_word_forward();
            Ok(None)
        }
        KeyCode::Char('b') => {
            state.editor.move_word_backward();
            Ok(None)
        }
        KeyCode::Char('G') => {
            state.editor.goto_last_line();
            Ok(None)
        }
        _ => Ok(None),
    }
}
