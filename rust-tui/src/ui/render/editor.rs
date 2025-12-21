use super::highlight_rust_line;
use crate::ui::theme;
use crate::ui::tui::{EditorMode, TuiState};
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_editor(frame: &mut Frame, area: Rect, state: &mut TuiState, is_active: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if is_active {
            Style::default().fg(theme::colors::PRIMARY)
        } else {
            Style::default().fg(theme::colors::MUTED)
        })
        .title(Span::styled(
            " Editor ",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let visible_height = inner.height as usize;
    state.editor.update_scroll(visible_height);

    // Line number width
    let line_count = state.editor.lines.len();
    let line_num_width = (line_count.to_string().len() + 2) as u16;

    let editor_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(line_num_width), Constraint::Min(10)])
        .split(inner);

    // Render line numbers
    let visible_lines: Vec<Line> = (state.editor.scroll_offset
        ..state.editor.scroll_offset + visible_height)
        .filter_map(|i| {
            if i < line_count {
                Some(Line::from(Span::styled(
                    format!("{:>width$} ", i + 1, width = line_num_width as usize - 2),
                    Style::default().fg(theme::colors::MUTED),
                )))
            } else {
                None
            }
        })
        .collect();

    let line_nums_widget = Paragraph::new(visible_lines);
    frame.render_widget(line_nums_widget, editor_chunks[0]);

    // Render code with syntax highlighting
    let code_lines: Vec<Line> = state
        .editor
        .lines
        .iter()
        .skip(state.editor.scroll_offset)
        .take(visible_height)
        .enumerate()
        .map(|(i, line)| {
            let actual_row = state.editor.scroll_offset + i;
            let is_cursor_line = actual_row == state.editor.cursor_row;

            // Check if this line is in the visual selection
            let in_visual_mode = state.mode == EditorMode::Visual;
            let visual_bounds = if in_visual_mode {
                Some(get_selection_bounds(state))
            } else {
                None
            };

            // Visual mode rendering with selection highlighting
            if in_visual_mode {
                render_visual_line(line, actual_row, &visual_bounds, state, is_cursor_line)
            } else if is_cursor_line
                && (state.mode == EditorMode::Insert || state.mode == EditorMode::Normal)
            {
                // Use different cursor colors for different modes
                let cursor_color = if state.mode == EditorMode::Insert {
                    theme::colors::SUCCESS // Green for Insert
                } else {
                    theme::colors::PRIMARY // Orange for Normal
                };

                // Use char indices for proper UTF-8 handling
                let chars: Vec<char> = line.chars().collect();
                let col = state.editor.cursor_col.min(chars.len());

                let mut spans = Vec::new();

                // Text before cursor
                if col > 0 {
                    let before: String = chars[..col].iter().collect();
                    spans.push(Span::styled(
                        before,
                        Style::default().fg(theme::colors::TEXT),
                    ));
                }

                // Cursor character (inverted colors)
                if col < chars.len() {
                    spans.push(Span::styled(
                        chars[col].to_string(),
                        Style::default()
                            .fg(theme::colors::BACKGROUND)
                            .bg(cursor_color),
                    ));
                    // Text after cursor
                    if col + 1 < chars.len() {
                        let after: String = chars[col + 1..].iter().collect();
                        spans.push(Span::styled(
                            after,
                            Style::default().fg(theme::colors::TEXT),
                        ));
                    }
                } else {
                    // At end of line, show a block cursor (space with background)
                    spans.push(Span::styled(
                        " ",
                        Style::default()
                            .fg(theme::colors::BACKGROUND)
                            .bg(cursor_color),
                    ));
                }
                Line::from(spans)
            } else {
                // Syntax highlighting
                highlight_rust_line(line, is_cursor_line)
            }
        })
        .collect();

    let code_widget = Paragraph::new(code_lines);
    frame.render_widget(code_widget, editor_chunks[1]);
}

pub fn get_selection_bounds(state: &TuiState) -> (usize, usize, usize, usize) {
    let cur_row = state.editor.cursor_row;
    let cur_col = state.editor.cursor_col;
    let start_row = state.visual_start_row;
    let start_col = state.visual_start_col;

    if start_row < cur_row || (start_row == cur_row && start_col <= cur_col) {
        (start_row, start_col, cur_row, cur_col)
    } else {
        (cur_row, cur_col, start_row, start_col)
    }
}

/// Render a line with visual selection highlighting
fn render_visual_line(
    line: &str,
    row: usize,
    bounds: &Option<(usize, usize, usize, usize)>,
    state: &TuiState,
    is_cursor_line: bool,
) -> Line<'static> {
    let chars: Vec<char> = line.chars().collect();
    let mut spans = Vec::new();

    // Get selection bounds
    let (start_row, start_col, end_row, end_col) = bounds.unwrap_or((0, 0, 0, 0));

    // Check if this row is in the selection range
    if row < start_row || row > end_row {
        // Not in selection, render normally
        return highlight_rust_line(line, is_cursor_line);
    }

    // Determine selection columns for this row
    let (sel_start, sel_end) = if start_row == end_row {
        // Single line selection
        (start_col, end_col + 1)
    } else if row == start_row {
        // First line of multi-line selection
        (start_col, chars.len())
    } else if row == end_row {
        // Last line of multi-line selection
        (0, end_col + 1)
    } else {
        // Middle line - fully selected
        (0, chars.len())
    };

    let sel_start = sel_start.min(chars.len());
    let sel_end = sel_end.min(chars.len());

    // Text before selection
    if sel_start > 0 {
        let before: String = chars[..sel_start].iter().collect();
        spans.push(Span::styled(
            before,
            Style::default().fg(theme::colors::TEXT),
        ));
    }

    // Selected text (highlighted with magenta background)
    if sel_start < sel_end {
        let selected: String = chars[sel_start..sel_end].iter().collect();
        spans.push(Span::styled(
            selected,
            Style::default()
                .fg(theme::colors::BACKGROUND)
                .bg(theme::colors::PRIMARY), // Orange highlight for selection
        ));
    }

    // Cursor at current position in visual mode
    let cursor_col = state.editor.cursor_col;
    if is_cursor_line && cursor_col >= sel_end && cursor_col < chars.len() {
        // Show cursor after selection if visible
        let cursor_char = chars[cursor_col].to_string();
        spans.push(Span::styled(
            cursor_char,
            Style::default()
                .fg(theme::colors::BACKGROUND)
                .bg(theme::colors::SUCCESS), // Green cursor
        ));
        if cursor_col + 1 < chars.len() {
            let after: String = chars[cursor_col + 1..].iter().collect();
            spans.push(Span::styled(
                after,
                Style::default().fg(theme::colors::TEXT),
            ));
        }
    } else if sel_end < chars.len() {
        // Text after selection
        let after: String = chars[sel_end..].iter().collect();
        spans.push(Span::styled(
            after,
            Style::default().fg(theme::colors::TEXT),
        ));
    }

    // Handle empty line with cursor
    if line.is_empty() && is_cursor_line {
        spans.push(Span::styled(
            " ",
            Style::default()
                .fg(theme::colors::BACKGROUND)
                .bg(theme::colors::SUCCESS),
        ));
    }

    Line::from(spans)
}
