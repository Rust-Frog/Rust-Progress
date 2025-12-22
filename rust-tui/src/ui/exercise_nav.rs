//! Exercise navigation and file operations

use std::fs;
use std::time::SystemTime;

use anyhow::Result;

use crate::exercise::RunnableExercise;
use crate::ui::{
    editor::TextEditor,
    state::{TuiState, ViewMode},
    theme,
};

impl<'a> TuiState<'a> {
    /// Get file modification time
    pub fn get_file_modified_time(path: &str) -> Option<SystemTime> {
        fs::metadata(path).ok()?.modified().ok()
    }

    /// Check if file was modified externally and reload if needed
    pub fn check_external_file_change(&mut self) -> Result<bool> {
        let current_modified = Self::get_file_modified_time(&self.file_path);

        if let (Some(last), Some(current)) = (self.last_file_modified, current_modified)
            && current > last
            && !self.modified
        {
            self.last_file_modified = Some(current);
            let content = fs::read_to_string(&self.file_path)?;
            self.editor = TextEditor::new(&content);
            self.output = format!("{} File changed externally, reloaded!", theme::icons::INFO);

            if self.auto_compile_on_change {
                self.compile()?;
            }
            return Ok(true);
        }
        self.last_file_modified = current_modified;
        Ok(false)
    }

    /// Save current file
    pub fn save(&mut self) -> Result<()> {
        let content = self.editor.content();
        fs::write(&self.file_path, &content)?;
        self.modified = false;
        self.last_file_modified = Self::get_file_modified_time(&self.file_path);
        self.output = format!("{} File saved!", theme::icons::DONE);
        Ok(())
    }

    /// Compile and run exercise
    pub fn compile(&mut self) -> Result<bool> {
        self.save()?;
        self.output = format!(
            "{} Checking {}...",
            theme::icons::COMPILING,
            self.file_path.split('/').next_back().unwrap_or("file")
        );
        self.output_scroll = 0;

        let success = self
            .app_state
            .current_exercise()
            .run_exercise(Some(&mut self.output_buffer), self.app_state.cmd_runner())?;

        if success {
            let current_ind = self.app_state.current_exercise_ind();
            self.app_state.set_status(current_ind, true)?;
            self.app_state.write()?;

            let solution_msg = if let Ok(Some(sol_path)) = self.app_state.current_solution_path() {
                format!("\n\nSolution available: {}", sol_path)
            } else {
                String::new()
            };

            if self.auto_advance {
                if let Some(next_pending) = self.find_next_pending_exercise() {
                    self.app_state.set_current_exercise_ind(next_pending)?;
                    self.reload_exercise()?;
                    self.output = format!(
                        "{} Complete! Auto-advanced to: {}",
                        theme::icons::DONE,
                        self.file_path.split('/').next_back().unwrap_or("next")
                    );
                } else {
                    self.output = format!(
                        "{} Congratulations! All exercises complete! 🎉",
                        theme::icons::DONE
                    );
                }
            } else {
                self.output = format!(
                    "{} Exercise passed! Press ']' for next.{}",
                    theme::icons::DONE,
                    solution_msg
                );
            }
            Ok(true)
        } else {
            self.output = String::from_utf8_lossy(&self.output_buffer).to_string();
            Ok(false)
        }
    }

    /// Find next pending (unsolved) exercise
    pub fn find_next_pending_exercise(&self) -> Option<usize> {
        let exercises = self.app_state.exercises();
        let current = self.app_state.current_exercise_ind();

        // Search from current+1 to end
        exercises
            .iter()
            .enumerate()
            .skip(current + 1)
            .find_map(|(i, ex)| if !ex.done { Some(i) } else { None })
            .or_else(|| {
                // Wrap around: search from 0 to current
                exercises.iter().take(current).position(|ex| !ex.done)
            })
    }

    /// Toggle solution panel visibility
    pub fn toggle_solution(&mut self) {
        if self.view_mode == ViewMode::WithSolution {
            self.view_mode = ViewMode::EditorOnly;
            self.solution_content = None;
        } else {
            match self.app_state.current_solution_path() {
                Ok(Some(solution_path)) => {
                    if let Ok(content) = fs::read_to_string(&solution_path) {
                        self.solution_content = Some(content);
                        self.view_mode = ViewMode::WithSolution;
                        self.output = format!(
                            "{} Solution loaded: {}",
                            theme::icons::SOLUTION,
                            solution_path
                        );
                    } else {
                        self.output =
                            format!("{} Could not read solution file", theme::icons::ERROR);
                    }
                }
                Ok(None) => {
                    self.output = format!(
                        "{} No solution available for this exercise",
                        theme::icons::ERROR
                    );
                }
                Err(e) => {
                    self.output = format!("{} Error loading solution: {}", theme::icons::ERROR, e);
                }
            }
        }
    }

    /// Go to next exercise
    pub fn next_exercise(&mut self) -> Result<()> {
        if self.app_state.current_exercise_ind() < self.app_state.exercises().len() - 1 {
            self.app_state
                .set_current_exercise_ind(self.app_state.current_exercise_ind() + 1)?;
            self.reload_exercise()?;
        }
        Ok(())
    }

    /// Go to previous exercise
    pub fn prev_exercise(&mut self) -> Result<()> {
        if self.app_state.current_exercise_ind() > 0 {
            self.app_state
                .set_current_exercise_ind(self.app_state.current_exercise_ind() - 1)?;
            self.reload_exercise()?;
        }
        Ok(())
    }

    /// Reload current exercise from disk
    pub fn reload_exercise(&mut self) -> Result<()> {
        let exercise = self.app_state.current_exercise();
        self.file_path = exercise.path.to_string();
        let content = fs::read_to_string(&self.file_path)?;
        self.editor = TextEditor::new(&content);
        self.modified = false;
        self.solution_content = None;
        self.view_mode = ViewMode::EditorOnly;
        self.last_file_modified = Self::get_file_modified_time(&self.file_path);
        self.output_scroll = 0;
        self.frog_step = 0;
        self.frog_scroll = 0;
        self.current_frog_steps = Self::load_frog_content(exercise.path, exercise.name);
        Ok(())
    }
}
