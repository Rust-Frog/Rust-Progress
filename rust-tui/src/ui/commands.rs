//! Command execution for the TUI

use anyhow::Result;

use crate::ui::{state::TuiState, theme};

impl TuiState<'_> {
    /// Execute a vim-style command
    pub fn execute_command(&mut self, cmd: &str) -> Result<Option<bool>> {
        match cmd.trim() {
            "w" => {
                self.save()?;
                Ok(Some(false))
            }
            "q" => {
                if self.modified {
                    self.output =
                        format!("{} Unsaved changes! Use :q! or :wq", theme::icons::ERROR);
                    Ok(Some(false))
                } else {
                    Ok(Some(true))
                }
            }
            "q!" => Ok(Some(true)),
            "wq" | "x" => {
                self.save()?;
                Ok(Some(true))
            }
            "c" => {
                self.compile()?;
                Ok(Some(false))
            }
            "h" | "hint" => {
                let hint = &self.app_state.current_exercise().hint;
                self.output = format!("{} {}", theme::icons::HINT, hint);
                Ok(Some(false))
            }
            "s" | "sol" | "solution" => {
                self.toggle_solution();
                Ok(Some(false))
            }
            "n" | "next" => {
                self.next_exercise()?;
                Ok(Some(false))
            }
            "p" | "prev" => {
                self.prev_exercise()?;
                Ok(Some(false))
            }
            "auto" => {
                self.auto_advance = !self.auto_advance;
                let status = if self.auto_advance { "ON" } else { "OFF" };
                self.output = format!("{} Auto-advance: {}", theme::icons::DONE, status);
                Ok(Some(false))
            }
            "watch" => {
                self.auto_compile_on_change = !self.auto_compile_on_change;
                let status = if self.auto_compile_on_change {
                    "ON"
                } else {
                    "OFF"
                };
                self.output = format!(
                    "{} Auto-compile on file change: {}",
                    theme::icons::DONE,
                    status
                );
                Ok(Some(false))
            }
            "r" | "reload" => {
                self.reload_exercise()?;
                self.output = format!("{} Exercise reloaded from disk", theme::icons::INFO);
                Ok(Some(false))
            }
            "reset" => {
                self.app_state.reset_current_exercise()?;
                self.reload_exercise()?;
                self.output = format!("{} Exercise reset to original", theme::icons::DONE);
                Ok(Some(false))
            }
            "help" => {
                self.view_mode = crate::ui::state::ViewMode::HelpModal;
                Ok(Some(false))
            }
            _ => {
                self.output = format!(
                    "{} Unknown command: {} (try :help)",
                    theme::icons::ERROR,
                    cmd
                );
                Ok(Some(false))
            }
        }
    }
}
