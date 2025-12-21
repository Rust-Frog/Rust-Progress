//! Theme and color definitions for the Rustlings TUI
//!
//! Modern, vibrant Dracula-inspired theme with Rust accents

use ratatui::style::{Color, Modifier, Style};

/// Modern Dracula-inspired color palette with Rust accents
pub mod colors {
    use ratatui::style::Color;

    // Primary colors - vibrant Rust theme
    pub const PRIMARY: Color = Color::Rgb(255, 121, 63); // Bright Rust Orange #FF793F
    pub const PRIMARY_DIM: Color = Color::Rgb(200, 80, 40); // Dimmed orange for accents
    pub const SECONDARY: Color = Color::Rgb(40, 42, 54); // Dracula Background #282A36
    pub const ACCENT: Color = Color::Rgb(255, 184, 108); // Warm Amber #FFB86C

    // Background colors
    pub const BG_DARK: Color = Color::Rgb(30, 31, 41); // Darker background
    pub const BG_MEDIUM: Color = Color::Rgb(40, 42, 54); // Normal background  
    pub const BG_LIGHT: Color = Color::Rgb(68, 71, 90); // Lighter background (panels)
    pub const BG_HIGHLIGHT: Color = Color::Rgb(55, 58, 75); // Selection highlight
    pub const BACKGROUND: Color = BG_DARK; // Alias for cursor rendering

    // Status colors - vibrant and clear
    pub const SUCCESS: Color = Color::Rgb(80, 250, 123); // Bright Green #50FA7B
    pub const WARNING: Color = Color::Rgb(255, 184, 108); // Amber #FFB86C
    pub const ERROR: Color = Color::Rgb(255, 85, 85); // Bright Red #FF5555
    pub const INFO: Color = Color::Rgb(139, 233, 253); // Cyan #8BE9FD

    // Syntax highlighting colors
    pub const KEYWORD: Color = Color::Rgb(255, 121, 198); // Pink #FF79C6
    pub const STRING: Color = Color::Rgb(241, 250, 140); // Yellow #F1FA8C
    pub const FUNCTION: Color = Color::Rgb(80, 250, 123); // Green #50FA7B
    pub const COMMENT: Color = Color::Rgb(98, 114, 164); // Gray #6272A4
    pub const NUMBER: Color = Color::Rgb(189, 147, 249); // Purple #BD93F9

    // Text colors
    pub const TEXT: Color = Color::Rgb(248, 248, 242); // Bright White #F8F8F2
    pub const TEXT_DIM: Color = Color::Rgb(189, 193, 215); // Dimmed text
    pub const MUTED: Color = Color::Rgb(98, 114, 164); // Gray #6272A4
    pub const BORDER: Color = Color::Rgb(98, 114, 164); // Visible borders #6272A4
    pub const BORDER_ACTIVE: Color = Color::Rgb(139, 233, 253); // Cyan active border
}

/// Status icons - modern Unicode symbols
pub mod icons {
    pub const DONE: &str = "✓";
    pub const CURRENT: &str = "▶";
    pub const PENDING: &str = "○";
    pub const ERROR: &str = "✗";
    pub const COMPILING: &str = "⚡";
    pub const HINT: &str = "💡";
    pub const SOLUTION: &str = "📖";
    pub const CRAB: &str = "🦀";
    pub const ARROW_RIGHT: &str = "→";
    pub const PROGRESS_FULL: &str = "█";
    pub const PROGRESS_EMPTY: &str = "░";
    pub const INFO: &str = "ℹ";
    pub const RELOAD: &str = "↻";
}

/// Header style - bold Rust orange
pub fn header_style() -> Style {
    Style::default()
        .fg(colors::PRIMARY)
        .bg(colors::BG_DARK)
        .add_modifier(Modifier::BOLD)
}

/// Title style for panels
pub fn title_style() -> Style {
    Style::default()
        .fg(colors::TEXT)
        .add_modifier(Modifier::BOLD)
}

/// Success messages
pub fn success_style() -> Style {
    Style::default()
        .fg(colors::SUCCESS)
        .add_modifier(Modifier::BOLD)
}

/// Error messages
pub fn error_style() -> Style {
    Style::default().fg(colors::ERROR)
}

/// Muted/secondary text
pub fn muted_style() -> Style {
    Style::default().fg(colors::MUTED)
}

/// Normal border
pub fn border_style() -> Style {
    Style::default().fg(colors::BORDER)
}

/// Active/focused border
pub fn active_border_style() -> Style {
    Style::default().fg(colors::PRIMARY)
}

/// Solution panel border
pub fn solution_border_style() -> Style {
    Style::default().fg(colors::INFO)
}

/// Editor panel style
pub fn editor_style() -> Style {
    Style::default().fg(colors::TEXT).bg(colors::BG_MEDIUM)
}

/// Output panel style  
pub fn output_style() -> Style {
    Style::default().fg(colors::TEXT_DIM).bg(colors::BG_DARK)
}

/// Mode indicator style (Normal/Insert)
pub fn mode_normal_style() -> Style {
    Style::default()
        .fg(colors::BG_DARK)
        .bg(colors::INFO)
        .add_modifier(Modifier::BOLD)
}

pub fn mode_insert_style() -> Style {
    Style::default()
        .fg(colors::BG_DARK)
        .bg(colors::SUCCESS)
        .add_modifier(Modifier::BOLD)
}

pub fn mode_command_style() -> Style {
    Style::default()
        .fg(colors::BG_DARK)
        .bg(colors::ACCENT)
        .add_modifier(Modifier::BOLD)
}

pub fn mode_visual_style() -> Style {
    Style::default()
        .fg(colors::BG_DARK)
        .bg(colors::PRIMARY)
        .add_modifier(Modifier::BOLD)
}

/// Progress bar style
pub fn progress_style() -> Style {
    Style::default().fg(colors::SUCCESS)
}

/// Keybinding hint style
pub fn hint_style() -> Style {
    Style::default().fg(colors::MUTED)
}

/// Generate a progress bar string
pub fn progress_bar(done: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return icons::PROGRESS_EMPTY.repeat(width);
    }
    let filled = (done * width) / total;
    let empty = width.saturating_sub(filled);
    format!(
        "{}{}",
        icons::PROGRESS_FULL.repeat(filled),
        icons::PROGRESS_EMPTY.repeat(empty)
    )
}
