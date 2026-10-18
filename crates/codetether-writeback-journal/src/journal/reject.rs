//! Remove pending operations and retain typed rejection evidence.
use super::WritebackJournal;
use crate::{JournalEntry, RejectReason, TxnId};
use chrono::Utc;
impl WritebackJournal {
    /// Mark a staged transaction as rejected with `reason`. The
    /// transaction is dropped from the pending set — a subsequent
    /// [`Self::commit`] on the same id returns `Err(RejectReason)`.
    /// # Examples
    /// ```rust
    /// use codetether_writeback_journal::{Op, RejectReason, WritebackJournal};
    /// let mut journal = WritebackJournal::new("session");
    /// let id = journal.stage(Op::Save);
    /// journal.reject(id, RejectReason::DestructiveOp);
    /// assert!(journal.commit(id).is_err());
    /// ```
    pub fn reject(&mut self, txn: TxnId, reason: RejectReason) {
        self.pending.remove(&txn);
        self.entries.push(JournalEntry::Rejected {
            txn,
            at: Utc::now(),
            reason,
        });
    }
}
