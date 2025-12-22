use crate::ui::tui::{EditorMode, TuiState, ViewMode};
use anyhow::Result;
use crossterm::event::{self, KeyCode};

pub fn handle_normal_mode(key: event::KeyEvent, state: &mut TuiState) -> Result<Option<bool>> {
    // If help modal is open, any key dismisses it
    if state.view_mode == ViewMode::HelpModal {
        state.view_mode = ViewMode::EditorOnly;
        return Ok(None);
    }

    // Handle pending key sequences (dd, yy, r<char>, daw, diw, caw, ciw)
    if !state.pending_keys.is_empty() {
        let pending = state.pending_keys.clone();

        if let KeyCode::Char(c) = key.code {
            match (pending.as_slice(), c) {
                // dd - delete line
                (&['d'], 'd') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(line) = state.editor.delete_line() {
                        state.yank_buffer = Some(line);
                    }
                    return Ok(None);
                }
                // yy - yank line
                (&['y'], 'y') => {
                    state.pending_keys.clear();
                    if let Some(line) = state.editor.get_current_line() {
                        state.yank_buffer = Some(line.clone());
                    }
                    return Ok(None);
                }
                // r<char> - replace char
                (&['r'], c) => {
                    state.pending_keys.clear();
                    state.modified = true;
                    state.editor.replace_char(c);
                    return Ok(None);
                }
                // d + a/i - start text object delete
                (&['d'], 'a') | (&['d'], 'i') | (&['c'], 'a') | (&['c'], 'i') => {
                    state.pending_keys.push(c);
                    return Ok(None);
                }
                // daw - delete around word
                (&['d', 'a'], 'w') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(deleted) = state.editor.delete_around_word() {
                        state.yank_buffer = Some(deleted);
                    }
                    return Ok(None);
                }
                // diw - delete inner word
                (&['d', 'i'], 'w') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(deleted) = state.editor.delete_inner_word() {
                        state.yank_buffer = Some(deleted);
                    }
                    return Ok(None);
                }
                // caw - change around word
                (&['c', 'a'], 'w') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(deleted) = state.editor.delete_around_word() {
                        state.yank_buffer = Some(deleted);
                    }
                    state.mode = EditorMode::Insert;
                    return Ok(None);
                }
                // ciw - change inner word
                (&['c', 'i'], 'w') => {
                    state.pending_keys.clear();
                    state.modified = true;
                    if let Some(deleted) = state.editor.delete_inner_word() {
                        state.yank_buffer = Some(deleted);
                    }
                    state.mode = EditorMode::Insert;
                    return Ok(None);
                }
                _ => {
                    state.pending_keys.clear();
                }
            }
        } else {
            state.pending_keys.clear();
        }
    }

    match key.code {
        KeyCode::Char(':') => {
            state.mode = EditorMode::Command;
            state.command_buffer.clear();
            Ok(None)
        }
        KeyCode::Char('i') => {
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char('v') => {
            state.mode = EditorMode::Visual;
            state.visual_start_row = state.editor.cursor_row;
            state.visual_start_col = state.editor.cursor_col;
            Ok(None)
        }
        KeyCode::Char('s') => {
            state.toggle_solution();
            Ok(None)
        }
        // Ctrl+O - Toggle expanded output view (remapped from 'o')
        KeyCode::Char('o') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
            state.view_mode = if state.view_mode == ViewMode::ExpandedOutput {
                ViewMode::EditorOnly
            } else {
                ViewMode::ExpandedOutput
            };
            Ok(None)
        }
        // Shift+F - Toggle Frog learning panel
        KeyCode::Char('F') if key.modifiers.contains(event::KeyModifiers::SHIFT) => {
            state.show_frog = !state.show_frog;
            Ok(None)
        }
        // Shift+Right - Next Frog step
        KeyCode::Right if key.modifiers.contains(event::KeyModifiers::SHIFT) => {
            if state.show_frog {
                state.next_frog_step();
            }
            Ok(None)
        }
        // Shift+Left - Previous Frog step
        KeyCode::Left if key.modifiers.contains(event::KeyModifiers::SHIFT) => {
            if state.show_frog {
                state.prev_frog_step();
            }
            Ok(None)
        }
        // Shift+Down - Scroll Frog content down
        KeyCode::Down if key.modifiers.contains(event::KeyModifiers::SHIFT) => {
            if state.show_frog {
                state.scroll_frog_down();
            }
            Ok(None)
        }
        // Shift+Up - Scroll Frog content up
        KeyCode::Up if key.modifiers.contains(event::KeyModifiers::SHIFT) => {
            if state.show_frog {
                state.scroll_frog_up();
            }
            Ok(None)
        }
        KeyCode::Char('h') | KeyCode::Left => {
            state.editor.move_left();
            Ok(None)
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state.editor.move_down();
            Ok(None)
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.editor.move_up();
            Ok(None)
        }
        KeyCode::Char('l') | KeyCode::Right => {
            state.editor.move_right();
            Ok(None)
        }
        KeyCode::Char('0') | KeyCode::Home => {
            state.editor.move_to_line_start();
            Ok(None)
        }
        KeyCode::Char('$') | KeyCode::End => {
            state.editor.move_to_line_end();
            Ok(None)
        }
        KeyCode::Char('w') => {
            state.editor.move_word_forward();
            Ok(None)
        }
        KeyCode::Char('b') => {
            state.editor.move_word_backward();
            Ok(None)
        }
        KeyCode::Char('g') => {
            state.pending_keys.push('g');
            Ok(None)
        }
        KeyCode::Char('G') => {
            state.editor.goto_last_line();
            Ok(None)
        }
        KeyCode::Char('o') => {
            state.modified = true;
            state.editor.open_line_below();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char('O') => {
            state.modified = true;
            state.editor.open_line_above();
            state.mode = EditorMode::Insert;
            Ok(None)
        }
        KeyCode::Char('x') => {
            state.modified = true;
            state.editor.delete();
            Ok(None)
        }
        KeyCode::Char('p') => {
            if let Some(text) = &state.yank_buffer {
                state.modified = true;
                // If it's a whole line (contains newline), insert below
                if text.contains('\n') || text.ends_with('\n') {
                    state.editor.insert_line_below(text.trim_end().to_string());
                } else {
                    // Just insert characters
                    for c in text.chars() {
                        state.editor.insert_char(c);
                    }
                }
            }
            Ok(None)
        }
        KeyCode::Char('d') => {
            state.pending_keys.push('d');
            Ok(None)
        }
        KeyCode::Char('y') => {
            state.pending_keys.push('y');
            Ok(None)
        }
        KeyCode::Char('r') => {
            state.pending_keys.push('r');
            Ok(None)
        }
        KeyCode::Char('c') => {
            // Change command (like cw)
            state.pending_keys.push('c');
            Ok(None)
        }
        KeyCode::Char('u') => {
            // Undo - not implemented yet
            Ok(None)
        }
        KeyCode::Char(']') => {
            state.next_exercise()?;
            Ok(None)
        }
        KeyCode::Char('[') => {
            state.prev_exercise()?;
            Ok(None)
        }
        KeyCode::Char('q') => Ok(Some(true)),
        // Scrolling output panel
        KeyCode::PageDown => {
            state.output_scroll = state.output_scroll.saturating_add(10);
            Ok(None)
        }
        KeyCode::PageUp => {
            state.output_scroll = state.output_scroll.saturating_sub(10);
            Ok(None)
        }
        KeyCode::Char('J') => {
            // Shift + J
            state.output_scroll = state.output_scroll.saturating_add(5);
            Ok(None)
        }
        KeyCode::Char('K') => {
            // Shift + K
            state.output_scroll = state.output_scroll.saturating_sub(5);
            Ok(None)
        }
        _ => Ok(None),
    }
}
