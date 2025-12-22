//! TUI state definitions

use std::time::{Instant, SystemTime};

use crate::ui::editor::TextEditor;

/// View mode for layout switching
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    EditorOnly,
    WithSolution,
    ExpandedOutput,
    HelpModal,
}

/// Editor mode (Vim-style)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
    Visual,
}

/// Main TUI state container
pub struct TuiState<'a> {
    pub app_state: &'a mut crate::app_state::AppState,
    pub editor: TextEditor,
    pub solution_content: Option<String>,
    pub output: String,
    pub output_buffer: Vec<u8>,
    pub mode: EditorMode,
    pub view_mode: ViewMode,
    pub command_buffer: String,
    pub modified: bool,
    pub file_path: String,
    pub output_scroll: u16,
    pub start_time: Instant,
    pub auto_advance: bool,
    pub last_file_modified: Option<SystemTime>,
    pub auto_compile_on_change: bool,
    pub yank_buffer: Option<String>,
    pub pending_keys: Vec<char>,
    pub visual_start_row: usize,
    pub visual_start_col: usize,
    pub show_frog: bool,
    pub frog_step: usize,
    pub current_frog_steps: Vec<String>,
    pub frog_scroll: usize,
    pub frog_content_height: usize,
    pub frog_visible_height: usize,
}
