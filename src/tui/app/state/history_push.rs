//! Composer-owned bounded command-history insertion.
impl super::super::AppState {
    pub fn push_history(&mut self, entry: String) {
        self.history.push_history(entry);
    }
}
