use super::strip_ansi_codes;
use crate::ui::theme;
use crate::ui::tui::{EditorMode, TuiState};
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_footer(frame: &mut Frame, area: Rect, state: &TuiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    // Output panel - strip ANSI codes for clean display
    let clean_output = strip_ansi_codes(&state.output);
    let output_lines: Vec<&str> = clean_output.lines().collect();
    let total_lines = output_lines.len();
    let visible_height = chunks[0].height.saturating_sub(2) as usize; // Account for borders

    // Clamp scroll to valid range
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

    // Show scroll position if scrolled
    let scroll_indicator = if scroll_pos > 0 {
        format!(
            " Output [{}/{}] ",
            scroll_pos + visible_height.min(total_lines),
            total_lines
        )
    } else {
        " Output ".to_string()
    };

    let output_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::colors::MUTED))
        .title(Span::styled(
            scroll_indicator,
            Style::default().fg(theme::colors::MUTED),
        ));

    let output = Paragraph::new(clean_output.as_str())
        .block(output_block)
        .style(output_style)
        .wrap(Wrap { trim: false })
        .scroll((scroll_pos as u16, 0));
    frame.render_widget(output, chunks[0]);

    // Progress bar with pulsing orange ball
    let done = state.app_state.n_done() as usize;
    let total = state.app_state.exercises().len();
    let progress_width = chunks[1].width.saturating_sub(10) as usize; // Leave room for percentage

    // Smooth pulsing animation for the ball
    let elapsed_ms = state.start_time.elapsed().as_millis();
    let pulse_phase = (elapsed_ms % 1000) as f32 / 1000.0; // 0.0 to 1.0 over 1 second

    // Sinusoidal pulse for smooth brightness transition
    let brightness = ((pulse_phase * std::f32::consts::PI * 2.0).sin() + 1.0) / 2.0; // 0.0 to 1.0
    let r = 255;
    let g = (80.0 + brightness * 100.0) as u8; // 80-180
    let b = (30.0 + brightness * 80.0) as u8; // 30-110
    let ball_color = Color::Rgb(r, g, b);

    let filled = if total > 0 {
        (done * progress_width) / total
    } else {
        0
    };
    let empty = progress_width.saturating_sub(filled);

    // Progress percentage
    let percent = if total > 0 { (done * 100) / total } else { 0 };
    let percent_str = format!(" {}% ", percent);

    // Build the clean progress line: [orange━━━●gray━━━] XX%
    let mut spans = vec![Span::styled(" ", Style::default())];

    // Filled portion (completed) - orange
    if filled > 0 {
        spans.push(Span::styled(
            "━".repeat(filled),
            Style::default().fg(theme::colors::PRIMARY), // Orange
        ));
    }

    // The pulsing ball
    spans.push(Span::styled(
        "●",
        Style::default().fg(ball_color).add_modifier(Modifier::BOLD),
    ));

    // Empty portion (remaining) - gray
    if empty > 0 {
        spans.push(Span::styled(
            "━".repeat(empty),
            Style::default().fg(theme::colors::MUTED), // Gray
        ));
    }

    // Percentage indicator
    spans.push(Span::styled(
        percent_str,
        Style::default().fg(if percent == 100 {
            theme::colors::SUCCESS
        } else {
            theme::colors::TEXT_DIM
        }),
    ));

    let progress_line = Line::from(spans);
    let progress_bar = Paragraph::new(progress_line);
    frame.render_widget(progress_bar, chunks[1]);

    // Status bar
    let mode_span = match state.mode {
        EditorMode::Normal => Span::styled(" NORMAL ", theme::mode_normal_style()),
        EditorMode::Insert => Span::styled(" INSERT ", theme::mode_insert_style()),
        EditorMode::Command => Span::styled(
            format!(" :{} ", state.command_buffer),
            theme::mode_command_style(),
        ),
        EditorMode::Visual => Span::styled(" VISUAL ", theme::mode_visual_style()),
    };

    let keybindings = if state.mode == EditorMode::Command {
        "Enter: run │ Esc: cancel"
    } else {
        "i: edit │ :c compile │ :h hint │ s: solution │ [/]: nav │ :help │ q: quit"
    };

    let status_line = Line::from(vec![
        mode_span,
        Span::styled(" ", Style::default()),
        Span::styled(keybindings, Style::default().fg(theme::colors::TEXT_DIM)),
    ]);
    let status_bar = Paragraph::new(status_line);
    frame.render_widget(status_bar, chunks[2]);
}
