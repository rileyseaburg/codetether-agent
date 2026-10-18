//! Opaque monotonic transaction identity.
use crate::WritebackJournal;
use serde::{Deserialize, Serialize};

/// Opaque handle returned from [`WritebackJournal::stage`] and required
/// by [`WritebackJournal::reject`] / [`WritebackJournal::commit`].
///
/// Monotonic per journal instance. Crashing between stage and commit
/// leaves a `Staged` entry in the journal — the next load can reconcile
/// or abandon it.
/// # Examples
/// ```rust
/// assert_eq!(codetether_writeback_journal::TxnId(7).0, 7);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxnId(pub u64);
