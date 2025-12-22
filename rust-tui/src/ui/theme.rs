//! Theme and color definitions for the Rustlings TUI

use ratatui::style::{Modifier, Style};

/// Dracula-based color palette
pub mod colors {
    use ratatui::style::Color;

    // Primary
    pub const PRIMARY: Color = Color::Rgb(255, 121, 63); // Rust orange
    pub const ACCENT: Color = Color::Rgb(255, 184, 108); // Amber

    // Backgrounds
    pub const BG_DARK: Color = Color::Rgb(30, 31, 41);
    pub const BG_LIGHT: Color = Color::Rgb(68, 71, 90);
    pub const BACKGROUND: Color = BG_DARK;

    // Status
    pub const SUCCESS: Color = Color::Rgb(80, 250, 123); // Green
    pub const WARNING: Color = Color::Rgb(255, 184, 108); // Amber
    pub const ERROR: Color = Color::Rgb(255, 85, 85); // Red
    pub const INFO: Color = Color::Rgb(139, 233, 253); // Cyan

    // Syntax
    pub const KEYWORD: Color = Color::Rgb(255, 121, 198); // Pink
    pub const STRING: Color = Color::Rgb(241, 250, 140); // Yellow
    pub const COMMENT: Color = Color::Rgb(98, 114, 164); // Gray
    pub const NUMBER: Color = Color::Rgb(189, 147, 249); // Purple

    // Text
    pub const TEXT: Color = Color::Rgb(248, 248, 242);
    pub const TEXT_DIM: Color = Color::Rgb(189, 193, 215);
    pub const MUTED: Color = Color::Rgb(98, 114, 164);
}

/// Status icons
pub mod icons {
    pub const DONE: &str = "✓";
    pub const ERROR: &str = "✗";
    pub const COMPILING: &str = "⚡";
    pub const HINT: &str = "💡";
    pub const SOLUTION: &str = "📖";
    pub const CRAB: &str = "🦀";
    pub const INFO: &str = "ℹ";
}

/// Mode indicator styles
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
