/// Simple text editor state
pub struct TextEditor {
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
}

impl TextEditor {
    pub fn new(content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Self {
            lines: if lines.is_empty() {
                vec![String::new()]
            } else {
                lines
            },
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0,
        }
    }

    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn move_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.clamp_col();
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor_row < self.lines.len().saturating_sub(1) {
            self.cursor_row += 1;
            self.clamp_col();
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.current_line_len();
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_col < self.current_line_len() {
            self.cursor_col += 1;
        } else if self.cursor_row < self.lines.len().saturating_sub(1) {
            self.cursor_row += 1;
            self.cursor_col = 0;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_row < self.lines.len() {
            let line = &mut self.lines[self.cursor_row];
            let byte_idx = Self::char_to_byte_idx(line, self.cursor_col);
            line.insert(byte_idx, c);
            self.cursor_col += 1;
        }
    }

    pub fn insert_newline(&mut self) {
        if self.cursor_row < self.lines.len() {
            let line = self.lines[self.cursor_row].clone();
            let byte_idx = Self::char_to_byte_idx(&line, self.cursor_col);
            let (before, after) = line.split_at(byte_idx);
            self.lines[self.cursor_row] = before.to_string();
            self.lines.insert(self.cursor_row + 1, after.to_string());
            self.cursor_row += 1;
            self.cursor_col = 0;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            let line = &mut self.lines[self.cursor_row];
            // Get byte index of the character before cursor
            let byte_idx = Self::char_to_byte_idx(line, self.cursor_col - 1);
            line.remove(byte_idx);
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].chars().count();
            self.lines[self.cursor_row].push_str(&current_line);
        }
    }

    pub fn delete(&mut self) {
        if self.cursor_col < self.current_line_len() {
            let line = &mut self.lines[self.cursor_row];
            let byte_idx = Self::char_to_byte_idx(line, self.cursor_col);
            line.remove(byte_idx);
        } else if self.cursor_row < self.lines.len() - 1 {
            let next_line = self.lines.remove(self.cursor_row + 1);
            self.lines[self.cursor_row].push_str(&next_line);
        }
    }

    pub fn current_line_len(&self) -> usize {
        self.lines
            .get(self.cursor_row)
            .map(|l| l.chars().count())
            .unwrap_or(0)
    }

    /// Convert a character index to a byte index in a string
    fn char_to_byte_idx(s: &str, char_idx: usize) -> usize {
        s.char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(s.len())
    }

    pub fn clamp_col(&mut self) {
        self.cursor_col = self.cursor_col.min(self.current_line_len());
    }

    pub fn update_scroll(&mut self, visible_height: usize) {
        if self.cursor_row < self.scroll_offset {
            self.scroll_offset = self.cursor_row;
        } else if self.cursor_row >= self.scroll_offset + visible_height {
            self.scroll_offset = self.cursor_row - visible_height + 1;
        }
    }

    // === Vim Movement Methods ===

    pub fn move_to_line_start(&mut self) {
        self.cursor_col = 0;
    }

    pub fn move_to_line_end(&mut self) {
        self.cursor_col = self.current_line_len();
    }

    pub fn move_to_first_non_whitespace(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_row) {
            self.cursor_col = line.chars().take_while(|c| c.is_whitespace()).count();
        }
    }

    pub fn move_word_forward(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            let mut col = self.cursor_col;

            // Skip current word (non-whitespace)
            while col < chars.len() && !chars[col].is_whitespace() {
                col += 1;
            }
            // Skip whitespace
            while col < chars.len() && chars[col].is_whitespace() {
                col += 1;
            }

            if col >= chars.len() && self.cursor_row < self.lines.len() - 1 {
                // Move to next line
                self.cursor_row += 1;
                self.cursor_col = 0;
                self.move_to_first_non_whitespace();
            } else {
                self.cursor_col = col;
            }
        }
    }

    pub fn move_word_backward(&mut self) {
        if self.cursor_col == 0 && self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.current_line_len();
        }

        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            let mut col = self.cursor_col.saturating_sub(1);

            // Skip whitespace backwards
            while col > 0 && chars.get(col).map_or(false, |c| c.is_whitespace()) {
                col -= 1;
            }
            // Skip word backwards
            while col > 0 && chars.get(col - 1).map_or(false, |c| !c.is_whitespace()) {
                col -= 1;
            }

            self.cursor_col = col;
        }
    }

    pub fn goto_first_line(&mut self) {
        self.cursor_row = 0;
        self.clamp_col();
    }

    pub fn goto_last_line(&mut self) {
        self.cursor_row = self.lines.len().saturating_sub(1);
        self.clamp_col();
    }

    // === Vim Editing Methods ===

    pub fn delete_line(&mut self) -> Option<String> {
        if self.lines.len() > 1 {
            let deleted = self.lines.remove(self.cursor_row);
            if self.cursor_row >= self.lines.len() {
                self.cursor_row = self.lines.len().saturating_sub(1);
            }
            self.clamp_col();
            Some(deleted)
        } else {
            // Last line - just clear it
            let deleted = std::mem::take(&mut self.lines[0]);
            self.cursor_col = 0;
            Some(deleted)
        }
    }

    pub fn open_line_below(&mut self) {
        self.lines.insert(self.cursor_row + 1, String::new());
        self.cursor_row += 1;
        self.cursor_col = 0;
    }

    pub fn open_line_above(&mut self) {
        self.lines.insert(self.cursor_row, String::new());
        self.cursor_col = 0;
    }

    pub fn get_current_line(&self) -> Option<&String> {
        self.lines.get(self.cursor_row)
    }

    pub fn insert_line_below(&mut self, content: String) {
        self.lines.insert(self.cursor_row + 1, content);
    }

    pub fn replace_char(&mut self, c: char) {
        if let Some(line) = self.lines.get_mut(self.cursor_row) {
            let mut chars: Vec<char> = line.chars().collect();
            if self.cursor_col < chars.len() {
                chars[self.cursor_col] = c;
                *line = chars.into_iter().collect();
            }
        }
    }

    /// Get the character at the current cursor position (for skip-over logic)
    pub fn char_at_cursor(&self) -> Option<char> {
        self.lines
            .get(self.cursor_row)
            .and_then(|line| line.chars().nth(self.cursor_col))
    }

    /// Delete inner word (diw) - just the word, not surrounding spaces
    pub fn delete_inner_word(&mut self) -> Option<String> {
        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            if chars.is_empty() || self.cursor_col >= chars.len() {
                return None;
            }

            // Find word boundaries
            let mut start = self.cursor_col;
            let mut end = self.cursor_col;

            // Expand left while we're on word chars
            while start > 0 && !chars[start - 1].is_whitespace() {
                start -= 1;
            }
            // Expand right while we're on word chars
            while end < chars.len() && !chars[end].is_whitespace() {
                end += 1;
            }

            // Extract and delete
            let deleted: String = chars[start..end].iter().collect();
            let new_line: String = chars[..start].iter().chain(chars[end..].iter()).collect();
            self.lines[self.cursor_row] = new_line;
            self.cursor_col = start;
            Some(deleted)
        } else {
            None
        }
    }

    /// Delete around word (daw) - word plus trailing/leading whitespace
    pub fn delete_around_word(&mut self) -> Option<String> {
        if let Some(line) = self.lines.get(self.cursor_row) {
            let chars: Vec<char> = line.chars().collect();
            if chars.is_empty() || self.cursor_col >= chars.len() {
                return None;
            }

            // Find word boundaries
            let mut start = self.cursor_col;
            let mut end = self.cursor_col;

            // Expand left while we're on word chars
            while start > 0 && !chars[start - 1].is_whitespace() {
                start -= 1;
            }
            // Expand right while we're on word chars
            while end < chars.len() && !chars[end].is_whitespace() {
                end += 1;
            }

            // Also include trailing whitespace (or leading if at end of line)
            let orig_end = end;
            while end < chars.len() && chars[end].is_whitespace() {
                end += 1;
            }
            // If no trailing whitespace, try leading
            if end == orig_end && start > 0 {
                while start > 0 && chars[start - 1].is_whitespace() {
                    start -= 1;
                }
            }

            // Extract and delete
            let deleted: String = chars[start..end].iter().collect();
            let new_line: String = chars[..start].iter().chain(chars[end..].iter()).collect();
            self.lines[self.cursor_row] = new_line;
            self.cursor_col = start.min(self.lines[self.cursor_row].len());
            Some(deleted)
        } else {
            None
        }
    }
}
