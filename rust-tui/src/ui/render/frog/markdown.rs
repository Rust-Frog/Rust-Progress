//! Markdown rendering for Frog learning panel

use crate::ui::theme;
use ratatui::prelude::*;

use super::syntax::highlight_rust_line;

/// Convert markdown text to styled Lines for display
pub fn render_markdown(text: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;

    for line in text.lines() {
        let trimmed = line.trim_end();

        // Toggle code block state
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        // Inside code block - render with syntax highlighting
        if in_code_block {
            let highlighted = highlight_rust_line(line);
            lines.push(Line::from(highlighted));
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
        if let Some(header_text) = trimmed.strip_prefix("# ") {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                header_text.to_string(),
                Style::default()
                    .fg(theme::colors::ACCENT)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )]));
            lines.push(Line::from(""));
            continue;
        }

        // H2 header
        if let Some(header_text) = trimmed.strip_prefix("## ") {
            lines.push(Line::from(vec![Span::styled(
                header_text.to_string(),
                Style::default()
                    .fg(theme::colors::PRIMARY)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(""));
            continue;
        }

        // H3 header
        if let Some(header_text) = trimmed.strip_prefix("### ") {
            lines.push(Line::from(vec![Span::styled(
                header_text.to_string(),
                Style::default()
                    .fg(theme::colors::INFO)
                    .add_modifier(Modifier::BOLD),
            )]));
            continue;
        }

        // Table row (starts with |)
        if trimmed.starts_with('|') {
            if trimmed.contains("---") {
                lines.push(Line::from(vec![Span::styled(
                    trimmed.replace("-", "─").replace("|", "│"),
                    Style::default().fg(theme::colors::BG_LIGHT),
                )]));
            } else {
                lines.push(Line::from(vec![Span::styled(
                    trimmed.replace("|", "│"),
                    Style::default().fg(theme::colors::TEXT),
                )]));
            }
            continue;
        }

        // Bullet list
        if let Some(list_text) = trimmed.strip_prefix("- ") {
            lines.push(Line::from(vec![
                Span::styled("  • ", Style::default().fg(theme::colors::INFO)),
                Span::raw(render_inline_formatting(list_text)),
            ]));
            continue;
        }

        // Box drawing characters
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

/// Strip inline markdown markers (bold, code)
fn render_inline_formatting(text: &str) -> String {
    text.replace("**", "").to_string()
}
