//! Integrated Vim-style editor for Rustlings exercises
//!
//! This module provides an in-terminal code editor using the edtui crate.

pub mod state;
pub mod view;

pub use state::EditorState;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::stdout;

use crate::app_state::AppState;

/// Run the integrated editor for the current exercise
pub fn run_editor(app_state: &AppState) -> Result<()> {
    // Create editor state
    let mut editor_state = EditorState::new(app_state)?;

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        // Render
        terminal.draw(|frame| {
            view::render_editor(frame, &mut editor_state);
        })?;

        // Handle events
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let Some(action) = view::handle_key_event(key, &mut editor_state) {
                    match action {
                        view::EditorAction::Quit => break,
                        view::EditorAction::ExecuteCommand(cmd) => {
                            if let Some(exit) = editor_state.execute_command(&cmd)? {
                                if exit {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
