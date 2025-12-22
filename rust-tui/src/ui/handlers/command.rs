use crate::ui::state::{EditorMode, TuiState};
use anyhow::Result;
use crossterm::event::{self, KeyCode};

pub fn handle_command_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
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
