//! Input-mode and suggestion updates after shared history navigation.
use crate::tui::models::InputMode;
use codetether_composer_view::HistoryRecall;
#[path = "history_push.rs"]
mod history_push;
impl super::AppState {
    pub fn history_prev(&mut self) -> bool {
        let Some(input) = self.history.history_prev() else {
            return false;
        };
        self.input = input;
        self.input_cursor = self.input_char_count();
        self.input_mode = if self.input.starts_with('/') {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        self.refresh_slash_suggestions();
        true
    }
    pub fn history_next(&mut self) -> bool {
        match self.history.history_next() {
            Some(HistoryRecall::Entry(input)) => self.input = input,
            Some(HistoryRecall::End) => self.input.clear(),
            None => return false,
        }
        self.input_cursor = self.input_char_count();
        self.input_mode = if self.input.is_empty() {
            InputMode::Normal
        } else if self.input.starts_with('/') {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        self.refresh_slash_suggestions();
        true
    }
}
