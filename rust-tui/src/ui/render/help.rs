use crate::ui::theme;
use ratatui::prelude::*;
use ratatui::widgets::*;

// Helper to create a section header
fn header(text: &str) -> Line<'static> {
    Line::from(vec![Span::styled(
        format!("  {text}"),
        Style::default()
            .fg(theme::colors::INFO)
            .add_modifier(Modifier::BOLD),
    )])
}

// Helper to create a key binding entry
fn key(binding: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {binding:<12}"),
            Style::default().fg(theme::colors::ACCENT),
        ),
        Span::raw(desc.to_string()),
    ])
}

pub fn render_help_modal(frame: &mut Frame, area: Rect) {
    // Calculate centered modal area (60% width, 70% height)
    let modal_width = (area.width * 60 / 100).clamp(50, 80);
    let modal_height = (area.height * 70 / 100).clamp(20, 35);
    let modal_x = area.x + (area.width.saturating_sub(modal_width)) / 2;
    let modal_y = area.y + (area.height.saturating_sub(modal_height)) / 2;
    let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

    frame.render_widget(Clear, modal_area);

    let help_text = vec![
        Line::from(vec![Span::styled(
            "   Rustlings TUI Help   ",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        header("NAVIGATION"),
        key("]", "Next exercise"),
        key("[", "Previous exercise"),
        key("Shift+J/K", "Scroll output down/up"),
        key("PgDn/PgUp", "Fast scroll output"),
        Line::from(""),
        header("EDITING"),
        key("i", "Enter Insert mode"),
        key("Esc", "Return to Normal mode"),
        key("h/j/k/l", "Vim cursor movement"),
        Line::from(""),
        header("COMMANDS"),
        key(":w", "Save file"),
        key(":c", "Compile/check"),
        key(":hint / h", "Show hint"),
        key(":sol / s", "Toggle solution view"),
        key("Shift+F", "Toggle 🐸 Frog panel"),
        key("Ctrl+O", "Expand output panel"),
        key(":auto", "Toggle auto-advance"),
        key(":watch", "Toggle auto-compile"),
        key(":reset", "Reset exercise"),
        key(":q / q", "Quit"),
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
