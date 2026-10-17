//! Read-only projection of the current question and its selections.
use super::Draft;
use crate::Question;
impl Draft {
    /// Current question, never absent.
    pub fn question(&self) -> &Question {
        &self.request.questions()[self.index]
    }
    /// Zero-based question index.
    pub fn index(&self) -> usize {
        self.index
    }
    /// Number of questions in the batch.
    pub fn total(&self) -> usize {
        self.request.questions().len()
    }
    /// Highlighted row; `options.len()` denotes custom text.
    pub fn cursor(&self) -> usize {
        self.cursors[self.index]
    }
    /// Selected option flags for the current question.
    pub fn selected(&self) -> &[bool] {
        &self.selected[self.index]
    }
    /// Current custom answer, possibly empty.
    pub fn custom(&self) -> &str {
        &self.custom[self.index]
    }
}
