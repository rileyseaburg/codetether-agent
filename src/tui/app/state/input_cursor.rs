//! Input cursor movement methods.
//!
//! Character-index-aware cursor operations (arrows, word jumps, home/end)
//! and visibility clamping for multi-byte UTF-8 input.

impl super::AppState {
    pub(crate) fn input_char_count(&self) -> usize {
        self.input.chars().count()
    }

    pub(crate) fn char_to_byte_index(&self, char_index: usize) -> usize {
        if char_index == 0 {
            return 0;
        }
        self.input
            .char_indices()
            .nth(char_index)
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| self.input.len())
    }

    fn char_at(&self, char_index: usize) -> Option<char> {
        self.input.chars().nth(char_index)
    }

    pub fn clamp_input_cursor(&mut self) {
        self.input_cursor = self.input_cursor.min(self.input_char_count());
    }

    #[allow(dead_code)]
    pub fn ensure_input_cursor_visible(&mut self, visible_width: usize) {
        if visible_width == 0 {
            self.input_scroll = self.input_cursor;
            return;
        }
        if self.input_cursor < self.input_scroll {
            self.input_scroll = self.input_cursor;
        } else if self.input_cursor >= self.input_scroll.saturating_add(visible_width) {
            self.input_scroll = self
                .input_cursor
                .saturating_sub(visible_width.saturating_sub(1));
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.input_cursor < self.input_char_count() {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_word_left(&mut self) {
        self.clamp_input_cursor();
        while self.input_cursor > 0
            && self
                .char_at(self.input_cursor - 1)
                .is_some_and(char::is_whitespace)
        {
            self.input_cursor -= 1;
        }
        while self.input_cursor > 0
            && self
                .char_at(self.input_cursor - 1)
                .is_some_and(|c| !c.is_whitespace())
        {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_word_right(&mut self) {
        self.clamp_input_cursor();
        while self.input_cursor < self.input_char_count()
            && self
                .char_at(self.input_cursor)
                .is_some_and(char::is_whitespace)
        {
            self.input_cursor += 1;
        }
        while self.input_cursor < self.input_char_count()
            && self
                .char_at(self.input_cursor)
                .is_some_and(|c| !c.is_whitespace())
        {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.input_cursor = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.input_cursor = self.input_char_count();
    }
}
