use crate::ui::render::editor::get_selection_bounds;
use crate::ui::state::{EditorMode, TuiState};
use anyhow::Result;
use crossterm::event::{self, KeyCode};

pub fn handle_visual_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match key.code {
        KeyCode::Esc => {
            state.mode = EditorMode::Normal;
            Ok(None)
        }
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
pub fn get_visual_selection(state: &TuiState) -> String {
    let (start_row, start_col, end_row, end_col) = get_selection_bounds(state);

    if start_row == end_row {
        // Single line selection
        if let Some(line) = state.editor.lines.get(start_row) {
            let line_chars: Vec<char> = line.chars().collect();
            let start = start_col.min(line_chars.len());
            let end = (end_col + 1).min(line_chars.len());
            return line_chars[start..end].iter().collect();
        }
    } else {
        // Multi-line selection
        let mut result = String::new();
        for row in start_row..=end_row {
            if let Some(line) = state.editor.lines.get(row) {
                let line_chars: Vec<char> = line.chars().collect();
                if row == start_row {
                    let start = start_col.min(line_chars.len());
                    let s: String = line_chars[start..].iter().collect();
                    result.push_str(&s);
                    result.push('\n');
                } else if row == end_row {
                    let end = (end_col + 1).min(line_chars.len());
                    let s: String = line_chars[..end].iter().collect();
                    result.push_str(&s);
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
pub fn delete_visual_selection(state: &mut TuiState) {
    let (start_row, start_col, end_row, end_col) = get_selection_bounds(state);

    // Yank before deleting
    let selected = get_visual_selection(state);
    state.yank_buffer = Some(selected);
    state.modified = true;

    if start_row == end_row {
        // Single line deletion
        if let Some(line) = state.editor.lines.get_mut(start_row) {
            let line_chars: Vec<char> = line.chars().collect();
            let start = start_col.min(line_chars.len());
            let end = (end_col + 1).min(line_chars.len());

            // Convert char indices back to byte indices for replace_range
            let byte_start: usize = line_chars[..start].iter().map(|c| c.len_utf8()).sum();
            let byte_end: usize = byte_start
                + line_chars[start..end]
                    .iter()
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
            line.replace_range(byte_start..byte_end, "");
        }
    } else {
        // Multi-line deletion: keep start of first line, end of last line, remove middle lines
        let first_part: String = state
            .editor
            .lines
            .get(start_row)
            .map(|l| {
                let chars: Vec<char> = l.chars().collect();
                chars[..start_col.min(chars.len())]
                    .iter()
                    .collect::<String>()
            })
            .unwrap_or_default();
        let last_part: String = state
            .editor
            .lines
            .get(end_row)
            .map(|l| {
                let chars: Vec<char> = l.chars().collect();
                chars[(end_col + 1).min(chars.len())..]
                    .iter()
                    .collect::<String>()
            })
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
