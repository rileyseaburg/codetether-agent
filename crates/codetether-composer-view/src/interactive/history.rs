//! Input-mode and suggestion updates after shared history navigation.
use crate::HistoryRecall;
use codetether_tui_models::InputMode;
impl super::Composer<'_> {
    /// Recall the previous input and update cursor, mode and completions.
    pub fn history_prev(&mut self) -> bool {
        let Some(input) = self.history.history_prev() else {
            return false;
        };
        *self.input = input;
        *self.cursor = crate::cursor_index::input_char_count(self.input);
        *self.mode = if self.input.starts_with('/') {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        self.refresh();
        true
    }
    /// Recall the next input or return to empty editing state.
    pub fn history_next(&mut self) -> bool {
        match self.history.history_next() {
            Some(HistoryRecall::Entry(input)) => *self.input = input,
            Some(HistoryRecall::End) => self.input.clear(),
            None => return false,
        }
        *self.cursor = crate::cursor_index::input_char_count(self.input);
        *self.mode = if self.input.is_empty() {
            InputMode::Normal
        } else if self.input.starts_with('/') {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        self.refresh();
        true
    }
}
