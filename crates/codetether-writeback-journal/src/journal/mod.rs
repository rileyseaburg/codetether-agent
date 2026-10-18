//! In-memory pending operations and ordered writeback evidence.
use crate::{JournalEntry, Op, TxnId};
use std::collections::HashMap;
mod commit;
mod construct;
mod query;
mod reject;
mod stage;

/// In-memory, per-session writeback journal.
///
/// Writes go into [`Self::entries`]; [`crate::append_entries`] persists a
/// caller-selected batch without coupling this state machine to file paths.
/// # Examples
/// ```rust
/// let mut journal = codetether_writeback_journal::WritebackJournal::new("session");
/// let id = journal.stage(codetether_writeback_journal::Op::Save);
/// journal.commit(id).unwrap();
/// assert_eq!(journal.pending_count(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct WritebackJournal {
    session_id: String,
    next_id: u64,
    pending: HashMap<TxnId, Op>,
    entries: Vec<JournalEntry>,
}
