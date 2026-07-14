use std::ops::Range;

impl super::AppState {
    pub fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str) {
        self.clamp_input_cursor();
        let start = self.char_to_byte_index(range.start.min(self.input_char_count()));
        let end = self.char_to_byte_index(range.end.min(self.input_char_count()));
        self.input.replace_range(start..end, replacement);
        self.input_cursor = range.start + replacement.chars().count();
        self.history_index = None;
        self.refresh_slash_suggestions();
    }
}
