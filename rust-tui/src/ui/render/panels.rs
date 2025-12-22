use super::{highlight_rust_line, strip_ansi_codes};
use crate::ui::state::TuiState;
use crate::ui::theme;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_solution(frame: &mut Frame, area: Rect, state: &TuiState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::INFO))
        .title(Span::styled(
            format!(" {} Solution ", theme::icons::SOLUTION),
            Style::default()
                .fg(theme::colors::INFO)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(ref content) = state.solution_content {
        let lines: Vec<Line> = content
            .lines()
            .map(|line| highlight_rust_line(line, false))
            .collect();
        let solution_widget = Paragraph::new(lines);
        frame.render_widget(solution_widget, inner);
    }
}

pub fn render_expanded_output(frame: &mut Frame, area: Rect, state: &TuiState) {
    // Full-screen output view
    let clean_output = strip_ansi_codes(&state.output);
    let output_lines: Vec<&str> = clean_output.lines().collect();
    let total_lines = output_lines.len();
    let visible_height = area.height.saturating_sub(2) as usize;

    let max_scroll = total_lines.saturating_sub(visible_height);
    let scroll_pos = (state.output_scroll as usize).min(max_scroll);

    let output_style = if state.output.contains("✓") || state.output.contains("complete") {
        Style::default().fg(theme::colors::SUCCESS)
    } else if state.output.contains("error") || state.output.contains("✗") {
        Style::default().fg(theme::colors::ERROR)
    } else if state.output.contains("💡") {
        Style::default().fg(theme::colors::ACCENT)
    } else {
        Style::default().fg(theme::colors::TEXT)
    };

    let scroll_indicator = format!(
        " Output [{}/{}] ← Press 'o' to collapse ",
        scroll_pos + visible_height.min(total_lines),
        total_lines
    );

    let output_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::PRIMARY))
        .title(Span::styled(
            scroll_indicator,
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ));

    let output = Paragraph::new(clean_output.as_str())
        .block(output_block)
        .style(output_style)
        .wrap(Wrap { trim: false })
        .scroll((scroll_pos as u16, 0));
    frame.render_widget(output, area);
}
