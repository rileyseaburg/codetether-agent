//! Recall an older history entry, stopping at the oldest item.
use crate::CommandHistory;
impl CommandHistory {
    /// Select and copy the preceding input, or the newest when recall begins.
    ///
    /// # Examples
    /// ```
    /// let mut history = codetether_composer_view::CommandHistory::default();
    /// history.push_history("hello".into());
    /// assert_eq!(history.history_prev().as_deref(), Some("hello"));
    /// ```
    pub fn history_prev(&mut self) -> Option<String> {
        if self.command_history.is_empty() {
            return None;
        }
        let history_len = self.command_history.len();
        let new_index = match self.history_index {
            Some(index) if index > 0 => Some(index - 1),
            Some(index) => Some(index),
            None => Some(history_len - 1),
        };
        if let Some(index) = new_index {
            self.history_index = Some(index);
            return Some(self.command_history[index].clone());
        }
        None
    }
}
