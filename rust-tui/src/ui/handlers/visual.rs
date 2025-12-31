use crate::ui::render::editor::get_selection_bounds;
use crate::ui::state::{EditorMode, TuiState};
use anyhow::Result;
use crossterm::event::{self, KeyCode};

// encapsulate selection range to reduce arg count
struct Bounds {
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
}

impl Bounds {
    fn from_state(state: &TuiState) -> Self {
        let (start_row, start_col, end_row, end_col) = get_selection_bounds(state);
        Self {
            start_row,
            start_col,
            end_row,
            end_col,
        }
    }

    fn is_single_line(&self) -> bool {
        self.start_row == self.end_row
    }
}

pub fn handle_visual_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match key.code {
        KeyCode::Esc => {
            state.mode = EditorMode::Normal;
            Ok(None)
        }

        // movement
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

        // actions
        KeyCode::Char('y') => {
            state.yank_buffer = Some(get_visual_selection(state));
            state.mode = EditorMode::Normal;
            Ok(None)
        }
        KeyCode::Char('d') => {
            delete_visual_selection(state);
            state.mode = EditorMode::Normal;
            Ok(None)
        }

        _ => Ok(None),
    }
}

// helper: get chars from a line, safely
fn line_chars(lines: &[String], row: usize) -> Vec<char> {
    lines
        .get(row)
        .map(|l| l.chars().collect())
        .unwrap_or_default()
}

// helper: slice chars from start to end (inclusive end_col)
fn slice_chars(chars: &[char], start: usize, end: usize) -> String {
    let s = start.min(chars.len());
    let e = end.min(chars.len());
    chars[s..e].iter().collect()
}

pub fn get_visual_selection(state: &TuiState) -> String {
    let b = Bounds::from_state(state);

    if b.is_single_line() {
        let chars = line_chars(&state.editor.lines, b.start_row);
        return slice_chars(&chars, b.start_col, b.end_col + 1);
    }

    // multi-line
    let mut result = String::new();
    for row in b.start_row..=b.end_row {
        let chars = line_chars(&state.editor.lines, row);

        let slice = if row == b.start_row {
            slice_chars(&chars, b.start_col, chars.len())
        } else if row == b.end_row {
            slice_chars(&chars, 0, b.end_col + 1)
        } else {
            chars.iter().collect()
        };

        result.push_str(&slice);
        if row != b.end_row {
            result.push('\n');
        }
    }
    result
}

pub fn delete_visual_selection(state: &mut TuiState) {
    let b = Bounds::from_state(state);

    // yank before delete
    state.yank_buffer = Some(get_visual_selection(state));
    state.modified = true;

    if b.is_single_line() {
        delete_single_line(state, &b);
    } else {
        delete_multi_line(state, &b);
    }

    state.editor.cursor_row = b.start_row;
    state.editor.cursor_col = b.start_col;
}

fn delete_single_line(state: &mut TuiState, b: &Bounds) {
    let Some(line) = state.editor.lines.get_mut(b.start_row) else {
        return;
    };

    let chars: Vec<char> = line.chars().collect();
    let start = b.start_col.min(chars.len());
    let end = (b.end_col + 1).min(chars.len());

    // char indices -> byte indices
    let byte_start: usize = chars[..start].iter().map(|c| c.len_utf8()).sum();
    let byte_end: usize = byte_start
        + chars[start..end]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();

    line.replace_range(byte_start..byte_end, "");
}

fn delete_multi_line(state: &mut TuiState, b: &Bounds) {
    // get the parts to keep
    let first_chars = line_chars(&state.editor.lines, b.start_row);
    let last_chars = line_chars(&state.editor.lines, b.end_row);

    let keep_first = slice_chars(&first_chars, 0, b.start_col);
    let keep_last = slice_chars(&last_chars, b.end_col + 1, last_chars.len());

    // remove middle lines
    for _ in b.start_row..b.end_row {
        if b.start_row + 1 < state.editor.lines.len() {
            state.editor.lines.remove(b.start_row + 1);
        }
    }

    // merge
    if let Some(line) = state.editor.lines.get_mut(b.start_row) {
        *line = keep_first + &keep_last;
    }
}
