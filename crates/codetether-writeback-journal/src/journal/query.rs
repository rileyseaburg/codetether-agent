//! Read journal ownership, ordered evidence and pending counts.
use super::WritebackJournal;
use crate::JournalEntry;
impl WritebackJournal {
    /// The session this journal belongs to.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Every entry in the order it was appended.
    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    /// Number of currently staged (neither committed nor rejected)
    /// transactions.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}
