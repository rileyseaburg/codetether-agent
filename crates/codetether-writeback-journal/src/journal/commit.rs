//! Commit only an operation that is still pending.
use super::WritebackJournal;
use crate::{JournalEntry, RejectReason, TxnId};
use chrono::Utc;
impl WritebackJournal {
    /// Commit a staged transaction that passed validation. Returns
    /// `Err(RejectReason::SchemaMismatch)` when the transaction is
    /// unknown (never staged, or already rejected / committed).
    ///
    /// # Errors
    ///
    /// See above — returns `Err` when the `TxnId` is not pending.
    pub fn commit(&mut self, txn: TxnId) -> Result<(), RejectReason> {
        if self.pending.remove(&txn).is_none() {
            return Err(RejectReason::SchemaMismatch {
                detail: format!("txn {:?} is not pending", txn),
            });
        }
        self.entries.push(JournalEntry::Committed {
            txn,
            at: Utc::now(),
        });
        Ok(())
    }
}
