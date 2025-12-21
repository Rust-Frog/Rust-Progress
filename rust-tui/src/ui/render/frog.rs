use crate::ui::theme;
use crate::ui::tui::TuiState;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_frog_panel(frame: &mut Frame, area: Rect, state: &TuiState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::SUCCESS))
        .title(Span::styled(
            " 🐸 Frog Learning Panel ",
            Style::default()
                .fg(theme::colors::SUCCESS)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if state.current_frog_steps.is_empty() {
        let text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "  No extra content for this exercise.",
                Style::default().fg(theme::colors::TEXT_DIM),
            )]),
        ];
        let content = Paragraph::new(text).style(Style::default().fg(theme::colors::TEXT));
        frame.render_widget(content, inner);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(inner);

    // 1. Render Step Bar
    let total_steps = state.current_frog_steps.len();
    let mut step_spans = Vec::new();

    step_spans.push(Span::raw("  ")); // Padding

    for i in 0..total_steps {
        let is_current = i == state.frog_step;
        let style = if is_current {
            Style::default()
                .fg(theme::colors::BG_DARK)
                .bg(theme::colors::ACCENT)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::colors::TEXT_DIM)
        };

        step_spans.push(Span::styled(format!(" {} ", i + 1), style));
        step_spans.push(Span::raw(" ")); // Spacing
    }

    let step_bar = Paragraph::new(Line::from(step_spans)).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme::colors::BG_LIGHT)),
    );
    frame.render_widget(step_bar, chunks[0]);

    // 2. Render Current Step Content
    if let Some(content_str) = state.current_frog_steps.get(state.frog_step) {
        let content_lines = render_markdown(content_str);
        let content = Paragraph::new(content_lines)
            .style(Style::default().fg(theme::colors::TEXT))
            .wrap(Wrap { trim: false })
            .scroll((state.frog_scroll as u16, 0));
        frame.render_widget(content, chunks[1]);
    }
}

fn render_markdown(text: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;

    for line in text.lines() {
        let trimmed = line.trim_end();

        // Toggle code block state
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            // Don't render the ``` markers themselves
            continue;
        }

        // Inside code block - render as code
        if in_code_block {
            lines.push(Line::from(vec![Span::styled(
                format!("  {}", line),
                Style::default().fg(theme::colors::STRING),
            )]));
            continue;
        }

        // Horizontal rule
        if trimmed.starts_with("---") && trimmed.chars().all(|c| c == '-' || c.is_whitespace()) {
            lines.push(Line::from(vec![Span::styled(
                "─".repeat(40),
                Style::default().fg(theme::colors::BG_LIGHT),
            )]));
            continue;
        }

        // H1 header
        if trimmed.starts_with("# ") {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                trimmed[2..].to_string(),
                Style::default()
                    .fg(theme::colors::ACCENT)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )]));
            lines.push(Line::from(""));
            continue;
        }

        // H2 header
        if trimmed.starts_with("## ") {
            lines.push(Line::from(vec![Span::styled(
                trimmed[3..].to_string(),
                Style::default()
                    .fg(theme::colors::PRIMARY)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(""));
            continue;
        }

        // H3 header
        if trimmed.starts_with("### ") {
            lines.push(Line::from(vec![Span::styled(
                trimmed[4..].to_string(),
                Style::default()
                    .fg(theme::colors::INFO)
                    .add_modifier(Modifier::BOLD),
            )]));
            continue;
        }

        // Table row (starts with |)
        if trimmed.starts_with('|') {
            // Check if it's a separator row (|---|---|)
            if trimmed.contains("---") {
                lines.push(Line::from(vec![Span::styled(
                    trimmed.replace("-", "─").replace("|", "│"),
                    Style::default().fg(theme::colors::BG_LIGHT),
                )]));
            } else {
                // Regular table row
                lines.push(Line::from(vec![Span::styled(
                    trimmed.replace("|", "│"),
                    Style::default().fg(theme::colors::TEXT),
                )]));
            }
            continue;
        }

        // Bullet list
        if trimmed.starts_with("- ") {
            lines.push(Line::from(vec![
                Span::styled("  • ", Style::default().fg(theme::colors::INFO)),
                Span::raw(render_inline_formatting(&trimmed[2..])),
            ]));
            continue;
        }

        // Box drawing characters - render with special color
        if trimmed.contains('┌')
            || trimmed.contains('│')
            || trimmed.contains('└')
            || trimmed.contains('├')
            || trimmed.contains('╭')
            || trimmed.contains('╰')
        {
            lines.push(Line::from(vec![Span::styled(
                trimmed.to_string(),
                Style::default().fg(theme::colors::INFO),
            )]));
            continue;
        }

        // Regular text with inline formatting
        let formatted = render_inline_formatting(trimmed);
        lines.push(Line::from(formatted));
    }

    lines
}

/// Handle inline formatting like **bold** and `code`
fn render_inline_formatting(text: &str) -> String {
    // For now, just strip ** markers for bold (can't easily style inline in ratatui)
    // and keep `code` as-is
    text.replace("**", "").to_string()
}
