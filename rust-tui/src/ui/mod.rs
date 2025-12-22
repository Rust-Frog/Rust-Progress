//! UI module for the Rustlings TUI

mod commands;
pub mod editor;
mod exercise_nav;
mod frog_state;
pub mod handlers;
pub mod layout;
pub mod render;
pub mod state;
pub mod theme;
pub mod tui;

// Types are accessed directly via crate::ui::state within UI modules
pub use tui::run_tui;
