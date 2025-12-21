//! Layout utilities for the Rustlings TUI
use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout mode for the main view
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Editor only with output at bottom
    EditorOnly,
    /// Editor left, solution right, output at bottom
    WithSolution,
}

/// Split the screen into header, main content, and output areas
pub fn main_layout(area: Rect) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Header
            Constraint::Min(10),    // Main content
            Constraint::Length(10), // Output + progress bar + status bar
        ])
        .split(area);

    (chunks[0], chunks[1], chunks[2])
}

/// Split main content for editor-only mode
pub fn editor_only_layout(area: Rect) -> Rect {
    area
}

/// Split main content for editor + solution mode
pub fn split_editors_layout(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    (chunks[0], chunks[1])
}

/// Create the footer/keybindings area
pub fn footer_layout(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    chunks[1]
}
