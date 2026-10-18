//! Serialized staging, rejection and commit evidence.
use crate::{Op, RejectReason, TxnId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single line in the journal.
/// # Examples
/// ```rust
/// use codetether_writeback_journal::{JournalEntry, TxnId};
/// let event = JournalEntry::Committed { txn: TxnId(0), at: chrono::Utc::now() };
/// assert!(matches!(event, JournalEntry::Committed { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum JournalEntry {
    /// An op was proposed and reserved a transaction id.
    Staged {
        txn: TxnId,
        at: DateTime<Utc>,
        op: Op,
    },
    /// Validation failed for a staged op. The transaction does not
    /// commit and the state does not mutate.
    Rejected {
        txn: TxnId,
        at: DateTime<Utc>,
        reason: RejectReason,
    },
    /// A staged op passed validation (or skipped it) and committed.
    Committed { txn: TxnId, at: DateTime<Utc> },
}
