//! Reserve transaction identities and record proposed operations.
use super::WritebackJournal;
use crate::{JournalEntry, Op, TxnId};
use chrono::Utc;
impl WritebackJournal {
    /// Stage an op and return its [`TxnId`]. Writes a `Staged` entry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_writeback_journal::{Op, WritebackJournal};
    ///
    /// let mut journal = WritebackJournal::new("s");
    /// let txn = journal.stage(Op::Save);
    /// assert_eq!(journal.entries().len(), 1);
    /// journal.commit(txn).unwrap();
    /// assert_eq!(journal.entries().len(), 2);
    /// ```
    /// # Panics
    /// Panics on transaction-counter overflow when integer overflow checks are enabled.
    pub fn stage(&mut self, op: Op) -> TxnId {
        let txn = TxnId(self.next_id);
        self.next_id += 1;
        self.pending.insert(txn, op.clone());
        self.entries.push(JournalEntry::Staged {
            txn,
            at: Utc::now(),
            op,
        });
        txn
    }
}
