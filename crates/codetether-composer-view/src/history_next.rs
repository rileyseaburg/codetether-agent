//! Recall a newer input or leave history browsing.
use crate::{CommandHistory, HistoryRecall};
impl CommandHistory {
    /// Select the next entry or report that the recalled input should clear.
    ///
    /// # Examples
    /// ```
    /// let mut history = codetether_composer_view::CommandHistory::default();
    /// history.push_history("hello".into());
    /// history.history_prev();
    /// assert!(matches!(history.history_next(), Some(codetether_composer_view::HistoryRecall::End)));
    /// ```
    pub fn history_next(&mut self) -> Option<HistoryRecall> {
        if self.command_history.is_empty() {
            return None;
        }
        match self.history_index {
            Some(index) if index + 1 < self.command_history.len() => {
                let next = index + 1;
                self.history_index = Some(next);
                Some(HistoryRecall::Entry(self.command_history[next].clone()))
            }
            Some(_) => {
                self.history_index = None;
                Some(HistoryRecall::End)
            }
            None => None,
        }
    }
}
