//! Frog Learning Panel rendering

mod markdown;
pub mod syntax;

use crate::ui::state::TuiState;
use crate::ui::theme;
use ratatui::prelude::*;
use ratatui::widgets::*;

use markdown::render_markdown;

/// Render the Frog learning panel
pub fn render_frog_panel(frame: &mut Frame, area: Rect, state: &mut TuiState) {
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

    // Step Bar
    render_step_bar(frame, chunks[0], state);

    // Content - clone to avoid borrow conflict
    if let Some(content_str) = state.current_frog_steps.get(state.frog_step).cloned() {
        render_content(frame, chunks[1], &content_str, state);
    }
}

fn render_step_bar(frame: &mut Frame, area: Rect, state: &TuiState) {
    let total_steps = state.current_frog_steps.len();
    let mut step_spans = Vec::new();
    step_spans.push(Span::raw("  "));

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
        step_spans.push(Span::raw(" "));
    }

    let step_bar = Paragraph::new(Line::from(step_spans)).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme::colors::BG_LIGHT)),
    );
    frame.render_widget(step_bar, area);
}

fn render_content(frame: &mut Frame, area: Rect, content_str: &str, state: &mut TuiState) {
    let content_lines = render_markdown(content_str);
    let total_lines = content_lines.len();
    let visible_height = area.height as usize;

    let max_scroll = total_lines.saturating_sub(visible_height);
    if state.frog_scroll > max_scroll {
        state.frog_scroll = max_scroll;
    }

    state.frog_content_height = total_lines;
    state.frog_visible_height = visible_height;

    let content = Paragraph::new(content_lines)
        .style(Style::default().fg(theme::colors::TEXT))
        .wrap(Wrap { trim: false })
        .scroll((state.frog_scroll as u16, 0));

    let content_area = if total_lines > visible_height {
        Rect {
            x: area.x,
            y: area.y,
            width: area.width.saturating_sub(2),
            height: area.height,
        }
    } else {
        area
    };

    frame.render_widget(content, content_area);

    // Scroll indicators
    if total_lines > visible_height {
        render_scroll_indicators(frame, area, state.frog_scroll, max_scroll);
    }
}

fn render_scroll_indicators(frame: &mut Frame, area: Rect, scroll: usize, max_scroll: usize) {
    let indicator_x = area.x + area.width - 1;

    if scroll > 0 {
        frame.render_widget(
            Paragraph::new("▲").style(Style::default().fg(theme::colors::ACCENT)),
            Rect {
                x: indicator_x,
                y: area.y,
                width: 1,
                height: 1,
            },
        );
    }

    if scroll < max_scroll {
        frame.render_widget(
            Paragraph::new("▼").style(Style::default().fg(theme::colors::ACCENT)),
            Rect {
                x: indicator_x,
                y: area.y + area.height - 1,
                width: 1,
                height: 1,
            },
        );
    }

    // Scrollbar
    let scrollbar_height = area.height.saturating_sub(2);
    if scrollbar_height > 0 {
        let scroll_progress = if max_scroll > 0 {
            scroll as f64 / max_scroll as f64
        } else {
            0.0
        };
        let thumb_pos = (scroll_progress * (scrollbar_height as f64 - 1.0)) as u16;

        for i in 0..scrollbar_height {
            let char = if i == thumb_pos { "█" } else { "░" };
            let color = if i == thumb_pos {
                theme::colors::ACCENT
            } else {
                theme::colors::BG_LIGHT
            };
            frame.render_widget(
                Paragraph::new(char).style(Style::default().fg(color)),
                Rect {
                    x: indicator_x,
                    y: area.y + 1 + i,
                    width: 1,
                    height: 1,
                },
            );
        }
    }
}
