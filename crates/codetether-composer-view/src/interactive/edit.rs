//! Application history and suggestion updates following shared text edits.
use codetether_tui_models::InputMode;
impl super::Composer<'_> {
    /// Insert one character and detach recalled history only when text changes.
    pub fn insert_char(&mut self, c: char) {
        let changed = crate::InputEdit::new(self.input, self.cursor).insert_char(c);
        if changed {
            self.history.history_index = None;
            self.refresh();
        }
    }
    /// Insert text and refresh completion candidates only when text changes.
    pub fn insert_text(&mut self, text: &str) {
        let changed = crate::InputEdit::new(self.input, self.cursor).insert_text(text);
        if changed {
            self.history.history_index = None;
            self.refresh();
        }
    }
    /// Delete the previous character and update history/completions when changed.
    pub fn delete_backspace(&mut self) {
        let changed = crate::InputEdit::new(self.input, self.cursor).delete_backspace();
        if changed {
            self.history.history_index = None;
            self.refresh();
        }
    }
    /// Delete the next character and update history/completions when changed.
    pub fn delete_forward(&mut self) {
        let changed = crate::InputEdit::new(self.input, self.cursor).delete_forward();
        if changed {
            self.history.history_index = None;
            self.refresh();
        }
    }
    /// Clear input, reset mode and scroll, and discard completion/history selection.
    pub fn clear_input(&mut self) {
        crate::InputEdit::new(self.input, self.cursor).clear();
        *self.scroll = 0;
        *self.mode = InputMode::Normal;
        self.completion.clear();
        self.history.history_index = None;
    }
}
