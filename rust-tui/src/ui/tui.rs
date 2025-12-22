//! TUI application entry point

use std::fs;
use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::app_state::AppState;
use crate::exercise::OUTPUT_CAPACITY;
use crate::ui::{
    editor::TextEditor,
    handlers, layout, render,
    state::{EditorMode, TuiState, ViewMode},
};

const FILE_WATCH_POLL_MS: u64 = 500;

impl<'a> TuiState<'a> {
    /// Create new TUI state for an exercise
    pub fn new(app_state: &'a mut AppState) -> Result<Self> {
        let exercise = app_state.current_exercise();
        let file_path = exercise.path.to_string();
        let exercise_name = exercise.name.to_string();
        let content = fs::read_to_string(&file_path)?;
        let editor = TextEditor::new(&content);
        let last_file_modified = Self::get_file_modified_time(&file_path);
        let frog_steps = Self::load_frog_content(&file_path, &exercise_name);

        Ok(Self {
            app_state,
            editor,
            solution_content: None,
            output: String::new(),
            output_buffer: Vec::with_capacity(OUTPUT_CAPACITY),
            mode: EditorMode::Normal,
            view_mode: ViewMode::EditorOnly,
            command_buffer: String::new(),
            modified: false,
            file_path,
            output_scroll: 0,
            start_time: Instant::now(),
            auto_advance: true,
            last_file_modified,
            auto_compile_on_change: true,
            yank_buffer: None,
            pending_keys: Vec::new(),
            visual_start_row: 0,
            visual_start_col: 0,
            show_frog: true,
            frog_step: 0,
            current_frog_steps: frog_steps,
            frog_scroll: 0,
            frog_content_height: 0,
            frog_visible_height: 0,
        })
    }
}

fn render(frame: &mut Frame, state: &mut TuiState) {
    let (header, main, footer) = layout::main_layout(frame.area());

    render::render_header(frame, header, state);

    match state.view_mode {
        ViewMode::EditorOnly => {
            if state.show_frog {
                let (editor_area, right_panel) = layout::split_editors_layout(main);
                render::render_editor(frame, editor_area, state, true);
                render::render_frog_panel(frame, right_panel, state);
            } else {
                render::render_editor(frame, main, state, true);
            }
        }
        ViewMode::WithSolution => {
            let (editor_area, right_panel) = layout::split_editors_layout(main);
            render::render_editor(frame, editor_area, state, true);
            render::render_solution(frame, right_panel, state);
        }
        ViewMode::ExpandedOutput => {
            render::render_expanded_output(frame, main, state);
        }
        ViewMode::HelpModal => {
            if state.show_frog {
                let (editor_area, right_panel) = layout::split_editors_layout(main);
                render::render_editor(frame, editor_area, state, true);
                render::render_frog_panel(frame, right_panel, state);
            } else {
                render::render_editor(frame, main, state, true);
            }
            render::render_help_modal(frame, main);
        }
    }

    render::render_footer(frame, footer, state);
}

fn handle_key(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    match state.mode {
        EditorMode::Normal => handlers::handle_normal_mode(key, state),
        EditorMode::Insert => handlers::handle_insert_mode(key, state),
        EditorMode::Command => handlers::handle_command_mode(key, state),
        EditorMode::Visual => handlers::handle_visual_mode(key, state),
    }
}

/// Run the TUI application
pub fn run_tui(app_state: &mut AppState) -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = TuiState::new(app_state)?;

    loop {
        terminal.draw(|frame| render(frame, &mut state))?;

        if event::poll(Duration::from_millis(FILE_WATCH_POLL_MS))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press
                        && let Some(should_quit) = handle_key(key, &mut state)?
                            && should_quit {
                                break;
                            }
                }
                Event::Mouse(mouse) => match mouse.kind {
                    event::MouseEventKind::ScrollDown => {
                        state.output_scroll = state.output_scroll.saturating_add(2);
                    }
                    event::MouseEventKind::ScrollUp => {
                        state.output_scroll = state.output_scroll.saturating_sub(2);
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            state.check_external_file_change()?;
        }
    }

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show
    )?;
    Ok(())
}
