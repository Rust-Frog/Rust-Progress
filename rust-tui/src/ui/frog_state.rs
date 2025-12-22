//! Frog learning panel state management

use std::fs;

use crate::ui::state::TuiState;

impl TuiState<'_> {
    /// Load Frog content for an exercise from markdown files
    pub fn load_frog_content(exercise_path: &str, exercise_name: &str) -> Vec<String> {
        let dir = std::path::Path::new(exercise_path)
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let possible_paths = [
            format!("rust-tui/frog/{}/{}.md", dir, exercise_name),
            format!("frog/{}/{}.md", dir, exercise_name),
        ];

        for path in possible_paths {
            if let Ok(content) = fs::read_to_string(&path) {
                return content
                    .split("--- slide ---")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }

        Vec::new()
    }

    /// Advance to next frog step or scroll to bottom first
    pub fn next_frog_step(&mut self) {
        let max_scroll = if self.frog_content_height > self.frog_visible_height {
            self.frog_content_height
                .saturating_sub(self.frog_visible_height)
        } else {
            0
        };

        if max_scroll > 0 && self.frog_scroll < max_scroll {
            self.frog_scroll = max_scroll;
            return;
        }

        if self.frog_step < self.current_frog_steps.len().saturating_sub(1) {
            self.frog_step += 1;
            self.frog_scroll = 0;
        }
    }

    /// Go to previous frog step
    pub fn prev_frog_step(&mut self) {
        if self.frog_step > 0 {
            self.frog_step -= 1;
            self.frog_scroll = 0;
        }
    }

    /// Scroll frog content up
    pub fn scroll_frog_up(&mut self) {
        if self.frog_scroll > 0 {
            self.frog_scroll -= 1;
        }
    }

    /// Scroll frog content down
    pub fn scroll_frog_down(&mut self) {
        self.frog_scroll += 1;
    }
}
