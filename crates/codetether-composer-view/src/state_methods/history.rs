//! Owned composer history operations.
impl crate::ComposerState {
    /// Recall the previous history entry, returning whether input changed.
    pub fn history_prev(&mut self) -> bool {
        self.borrow().history_prev()
    }
    /// Recall the next history entry, returning whether input changed.
    pub fn history_next(&mut self) -> bool {
        self.borrow().history_next()
    }
    /// Record a submitted prompt in the bounded command history.
    pub fn push_history(&mut self, entry: ::std::string::String) {
        self.history.push_history(entry);
    }
}
