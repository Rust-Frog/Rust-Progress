use crate::ui::theme;
use crate::ui::tui::TuiState;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render_header(frame: &mut Frame, area: Rect, state: &TuiState) {
    let done = state.app_state.n_done();
    let total = state.app_state.exercises().len();
    let current = state.app_state.current_exercise_ind() + 1;
    let exercise_name = state.file_path.split('/').last().unwrap_or("unknown");
    let modified_indicator = if state.modified { " ●" } else { "" };

    let is_done = state.app_state.exercises()[state.app_state.current_exercise_ind()].done;
    let exercise_style = if is_done {
        Style::default()
            .fg(theme::colors::SUCCESS)
            .add_modifier(Modifier::BOLD | Modifier::CROSSED_OUT)
    } else {
        Style::default()
            .fg(theme::colors::TEXT)
            .add_modifier(Modifier::BOLD)
    };

    let header_line = Line::from(vec![
        Span::styled(
            format!(" {} ", theme::icons::CRAB),
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "RUSTLINGS",
            Style::default()
                .fg(theme::colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" │ ", Style::default().fg(theme::colors::MUTED)),
        Span::styled(
            format!("{}{}", exercise_name, modified_indicator),
            exercise_style,
        ),
        Span::styled(" │ ", Style::default().fg(theme::colors::MUTED)),
        Span::styled(
            format!("Exercise {}/{}", current, total),
            Style::default().fg(theme::colors::TEXT_DIM),
        ),
        Span::styled(" │ ", Style::default().fg(theme::colors::MUTED)),
        Span::styled(
            format!("{} done", done),
            Style::default()
                .fg(theme::colors::SUCCESS)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let header = Paragraph::new(header_line);
    frame.render_widget(header, area);
}
