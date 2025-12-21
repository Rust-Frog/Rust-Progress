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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner);

    // Placeholder content for now - will be expanded in later phases
    let text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            format!("  {} LEARNING PROGRESS", theme::icons::CRAB),
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Current Step: "),
            Span::styled(
                format!("{}", state.frog_step + 1),
                Style::default()
                    .fg(theme::colors::ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from("  This panel provides extra context"),
        Line::from("  and hints to help you understand"),
        Line::from("  the current exercise."),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  CONTROLS:",
            Style::default().fg(theme::colors::INFO),
        )]),
        Line::from(vec![
            Span::styled("  Shift+N ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Next step"),
        ]),
        Line::from(vec![
            Span::styled("  Shift+P ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Previous step"),
        ]),
        Line::from(vec![
            Span::styled("  Shift+F ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle panel"),
        ]),
        Line::from(vec![
            Span::styled("  s       ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Switch to solution"),
        ]),
    ];

    let content = Paragraph::new(text).style(Style::default().fg(theme::colors::TEXT));
    frame.render_widget(content, chunks[0]);
}
