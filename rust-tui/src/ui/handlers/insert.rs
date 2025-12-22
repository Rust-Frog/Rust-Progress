use crate::ui::state::{EditorMode, TuiState};
use anyhow::Result;
use crossterm::event::{self, KeyCode};

pub fn handle_insert_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
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
