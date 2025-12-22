use crate::ui::theme;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_help_modal(frame: &mut Frame, area: Rect) {
    // Calculate centered modal area (60% width, 70% height)
    let modal_width = (area.width * 60 / 100).max(50).min(80);
    let modal_height = (area.height * 70 / 100).max(20).min(35);
    let modal_x = area.x + (area.width.saturating_sub(modal_width)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_height)) / 2;
    let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

    // Clear the area behind the modal
    frame.render_widget(Clear, modal_area);

    // Build help content
    let help_text = vec![
        Line::from(vec![Span::styled(
            "   Rustlings TUI Help   ",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  NAVIGATION",
            Style::default()
                .fg(theme::colors::INFO)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ]         ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Next exercise"),
        ]),
        Line::from(vec![
            Span::styled("  [         ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Previous exercise"),
        ]),
        Line::from(vec![
            Span::styled("  Shift+J/K ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Scroll output down/up"),
        ]),
        Line::from(vec![
            Span::styled("  PgDn/PgUp ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Fast scroll output"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  EDITING",
            Style::default()
                .fg(theme::colors::INFO)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  i         ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Enter Insert mode"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Return to Normal mode"),
        ]),
        Line::from(vec![
            Span::styled("  h/j/k/l   ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Vim cursor movement"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  COMMANDS",
            Style::default()
                .fg(theme::colors::INFO)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  :w        ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Save file"),
        ]),
        Line::from(vec![
            Span::styled("  :c        ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Compile/check"),
        ]),
        Line::from(vec![
            Span::styled("  :hint / h ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Show hint"),
        ]),
        Line::from(vec![
            Span::styled("  :sol / s  ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle solution view"),
        ]),
        Line::from(vec![
            Span::styled("  Shift+F   ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle 🐸 Frog panel"),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+O    ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Expand output panel"),
        ]),
        Line::from(vec![
            Span::styled("  :auto     ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle auto-advance"),
        ]),
        Line::from(vec![
            Span::styled("  :watch    ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Toggle auto-compile"),
        ]),
        Line::from(vec![
            Span::styled("  :reset    ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Reset exercise"),
        ]),
        Line::from(vec![
            Span::styled("  :q / q    ", Style::default().fg(theme::colors::ACCENT)),
            Span::raw("Quit"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Press Esc or any key to close",
            Style::default().fg(theme::colors::MUTED),
        )]),
    ];

    let help_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::PRIMARY))
        .title(Span::styled(
            " 🦀 Help ",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    let help_paragraph = Paragraph::new(help_text)
        .block(help_block)
        .style(Style::default().fg(theme::colors::TEXT));

    frame.render_widget(help_paragraph, modal_area);
}
