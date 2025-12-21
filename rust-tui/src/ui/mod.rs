//! UI module for the Rustlings TUI
//!
//! Contains theme, layout, and reusable components.

pub mod editor;
pub mod handlers;
pub mod layout;
pub mod render;
pub mod theme;
pub mod tui;

pub use tui::run_tui;
