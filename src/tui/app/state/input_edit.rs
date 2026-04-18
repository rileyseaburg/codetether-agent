//! Input text editing methods (insert, delete, clear).

use crate::tui::models::InputMode;

impl super::AppState {
    pub fn insert_char(&mut self, c: char) {
        self.clamp_input_cursor();
        let byte_index = self.char_to_byte_index(self.input_cursor);
        self.input.insert(byte_index, c);
        self.input_cursor += 1;
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn insert_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        self.clamp_input_cursor();
        let byte_index = self.char_to_byte_index(self.input_cursor);
        self.input.insert_str(byte_index, text);
        self.input_cursor += text.chars().count();
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn delete_backspace(&mut self) {
        self.clamp_input_cursor();
        if self.input_cursor == 0 {
            return;
        }
        let start = self.char_to_byte_index(self.input_cursor - 1);
        let end = self.char_to_byte_index(self.input_cursor);
        self.input.replace_range(start..end, "");
        self.input_cursor -= 1;
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn delete_forward(&mut self) {
        self.clamp_input_cursor();
        if self.input_cursor >= self.input_char_count() {
            return;
        }
        let start = self.char_to_byte_index(self.input_cursor);
        let end = self.char_to_byte_index(self.input_cursor + 1);
        self.input.replace_range(start..end, "");
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.input_cursor = 0;
        self.input_scroll = 0;
        self.input_mode = InputMode::Normal;
        self.slash_suggestions.clear();
        self.selected_slash_suggestion = 0;
        self.history_index = None;
    }
}
