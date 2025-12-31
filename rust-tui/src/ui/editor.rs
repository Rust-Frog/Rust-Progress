// don't blow up memory with infinite undo
const MAX_UNDO_HISTORY: usize = 100;

#[derive(Clone)]
struct EditorSnapshot {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
}

// encapsulates bracket matching state
struct BracketMatch {
    open: char,
    close: char,
    depth: i32,
}

impl BracketMatch {
    fn new(open: char, close: char) -> Self {
        Self {
            open,
            close,
            depth: 1,
        }
    }

    // returns Some(col) when matching bracket found
    fn check(&mut self, c: char, col: usize) -> Option<usize> {
        if c == self.open {
            self.depth += 1;
        } else if c == self.close {
            self.depth -= 1;
            if self.depth == 0 {
                return Some(col);
            }
        }
        None
    }

    fn scan_line(&mut self, chars: &[char], start: usize, forward: bool) -> Option<usize> {
        if forward {
            for (col, &c) in chars.iter().enumerate().skip(start) {
                if let Some(result) = self.check(c, col) {
                    return Some(result);
                }
            }
        } else {
            // when going backward, swap open/close
            std::mem::swap(&mut self.open, &mut self.close);
            for col in (0..=start).rev() {
                if let Some(result) = self.check(chars[col], col) {
                    return Some(result);
                }
            }
            std::mem::swap(&mut self.open, &mut self.close);
        }
        None
    }
}

pub struct TextEditor {
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
    undo_stack: Vec<EditorSnapshot>,
    redo_stack: Vec<EditorSnapshot>,
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
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    fn create_snapshot(&self) -> EditorSnapshot {
        EditorSnapshot {
            lines: self.lines.clone(),
            cursor_row: self.cursor_row,
            cursor_col: self.cursor_col,
        }
    }

    fn restore_snapshot(&mut self, snapshot: EditorSnapshot) {
        self.lines = snapshot.lines;
        self.cursor_row = snapshot.cursor_row;
        self.cursor_col = snapshot.cursor_col;
    }

    pub fn save_snapshot(&mut self) {
        let snapshot = self.create_snapshot();
        self.undo_stack.push(snapshot);

        if self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }

        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> bool {
        if let Some(snapshot) = self.undo_stack.pop() {
            self.redo_stack.push(self.create_snapshot());
            self.restore_snapshot(snapshot);
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(snapshot) = self.redo_stack.pop() {
            self.undo_stack.push(self.create_snapshot());
            self.restore_snapshot(snapshot);
            true
        } else {
            false
        }
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

    // char idx -> byte idx
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

    // -- vim movement --

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
            while col > 0 && chars.get(col).is_some_and(|c| c.is_whitespace()) {
                col -= 1;
            }
            // Skip word backwards
            while col > 0 && chars.get(col - 1).is_some_and(|c| !c.is_whitespace()) {
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

    // -- vim editing --

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

    pub fn char_at_cursor(&self) -> Option<char> {
        self.lines
            .get(self.cursor_row)
            .and_then(|line| line.chars().nth(self.cursor_col))
    }

    fn find_word_boundaries(&self, chars: &[char]) -> (usize, usize) {
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
        (start, end)
    }

    fn delete_range(&mut self, chars: &[char], start: usize, end: usize) -> String {
        let deleted: String = chars[start..end].iter().collect();
        let new_line: String = chars[..start].iter().chain(chars[end..].iter()).collect();
        self.lines[self.cursor_row] = new_line;
        deleted
    }

    // diw
    pub fn delete_inner_word(&mut self) -> Option<String> {
        let line = self.lines.get(self.cursor_row)?;
        let chars: Vec<char> = line.chars().collect();
        if chars.is_empty() || self.cursor_col >= chars.len() {
            return None;
        }

        let (start, end) = self.find_word_boundaries(&chars);
        let deleted = self.delete_range(&chars, start, end);
        self.cursor_col = start;
        Some(deleted)
    }

    // daw
    pub fn delete_around_word(&mut self) -> Option<String> {
        let line = self.lines.get(self.cursor_row)?;
        let chars: Vec<char> = line.chars().collect();
        if chars.is_empty() || self.cursor_col >= chars.len() {
            return None;
        }

        let (mut start, mut end) = self.find_word_boundaries(&chars);

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

        let deleted = self.delete_range(&chars, start, end);
        self.cursor_col = start.min(self.lines[self.cursor_row].len());
        Some(deleted)
    }

    // % - find matching bracket
    pub fn find_matching_bracket(&self) -> Option<(usize, usize)> {
        let char_at_cursor = self.char_at_cursor()?;

        // Define bracket pairs: (open, close, search_forward)
        let (open, close, forward) = match char_at_cursor {
            '(' => ('(', ')', true),
            ')' => ('(', ')', false),
            '{' => ('{', '}', true),
            '}' => ('{', '}', false),
            '[' => ('[', ']', true),
            ']' => ('[', ']', false),
            '<' => ('<', '>', true),
            '>' => ('<', '>', false),
            _ => return None,
        };

        if forward {
            self.search_bracket_forward(open, close)
        } else {
            self.search_bracket_backward(open, close)
        }
    }

    fn search_bracket_forward(&self, open: char, close: char) -> Option<(usize, usize)> {
        let mut matcher = BracketMatch::new(open, close);
        let mut row = self.cursor_row;
        let mut col = self.cursor_col + 1;

        while row < self.lines.len() {
            let chars: Vec<char> = self.lines[row].chars().collect();
            if let Some(result) = matcher.scan_line(&chars, col, true) {
                return Some((row, result));
            }
            row += 1;
            col = 0;
        }
        None
    }

    fn search_bracket_backward(&self, open: char, close: char) -> Option<(usize, usize)> {
        let mut matcher = BracketMatch::new(open, close);
        let mut row = self.cursor_row as i32;
        let mut col = self.cursor_col as i32 - 1;

        while row >= 0 {
            if col < 0 {
                row -= 1;
                col = self.get_line_end_col(row);
                continue;
            }

            let chars: Vec<char> = self.lines[row as usize].chars().collect();
            if let Some(result) = matcher.scan_line(&chars, col as usize, false) {
                return Some((row as usize, result));
            }

            row -= 1;
            col = self.get_line_end_col(row);
        }
        None
    }

    fn get_line_end_col(&self, row: i32) -> i32 {
        if row >= 0 {
            self.lines[row as usize].chars().count() as i32 - 1
        } else {
            -1
        }
    }
}
