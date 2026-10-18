//! Application history and suggestion updates following shared text edits.
use crate::tui::models::InputMode;
impl super::AppState {
    pub fn insert_char(&mut self, c: char) {
        let changed =
            codetether_composer_view::InputEdit::new(&mut self.input, &mut self.input_cursor)
                .insert_char(c);
        if changed {
            self.history_index = None;
            self.refresh_slash_suggestions();
        }
    }
    pub fn insert_text(&mut self, text: &str) {
        let changed =
            codetether_composer_view::InputEdit::new(&mut self.input, &mut self.input_cursor)
                .insert_text(text);
        if changed {
            self.history_index = None;
            self.refresh_slash_suggestions();
        }
    }
    pub fn delete_backspace(&mut self) {
        let changed =
            codetether_composer_view::InputEdit::new(&mut self.input, &mut self.input_cursor)
                .delete_backspace();
        if changed {
            self.history_index = None;
            self.refresh_slash_suggestions();
        }
    }
    pub fn delete_forward(&mut self) {
        let changed =
            codetether_composer_view::InputEdit::new(&mut self.input, &mut self.input_cursor)
                .delete_forward();
        if changed {
            self.history_index = None;
            self.refresh_slash_suggestions();
        }
    }
    pub fn clear_input(&mut self) {
        codetether_composer_view::InputEdit::new(&mut self.input, &mut self.input_cursor).clear();
        self.input_scroll = 0;
        self.input_mode = InputMode::Normal;
        self.slash_suggestions.clear();
        self.selected_slash_suggestion = 0;
        self.history_index = None;
    }
}
