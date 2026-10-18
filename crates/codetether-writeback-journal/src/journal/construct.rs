//! Construct an empty per-session writeback journal.
use super::WritebackJournal;
use std::collections::HashMap;
impl WritebackJournal {
    /// Create a fresh journal for `session_id`.
    /// # Examples
    /// ```rust
    /// let journal = codetether_writeback_journal::WritebackJournal::new("session");
    /// assert_eq!(journal.session_id(), "session");
    /// assert!(journal.entries().is_empty());
    /// ```
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            next_id: 0,
            pending: HashMap::new(),
            entries: Vec::new(),
        }
    }
}
