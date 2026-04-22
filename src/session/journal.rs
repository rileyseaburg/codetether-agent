//! Validated-writeback journal (ClawVM §3).
//!
//! Every lifecycle transition that touches durable state — compaction,
//! save, reset — goes through a three-phase transaction:
//!
//! 1. **Staging.** The caller proposes a typed [`Op`]; the journal
//!    reserves a [`TxnId`] and records the staged entry.
//! 2. **Validation.** The caller checks schema, provenance, scope,
//!    non-destructive semantics, and page-invariant compliance. The
//!    outcome is recorded.
//! 3. **Commit.** A validated transaction commits; a rejected one
//!    stays in the journal with its reason code and does *not* mutate
//!    the owning state.
//!
//! Rejections are not errors — they are load-bearing evidence that the
//! enforcement layer caught a would-be regression. ClawVM's LRU
//! ablation shows fault elimination comes from this structural
//! contract, not from clever selection heuristics.
//!
//! ## Format
//!
//! The journal is an append-only JSONL file named
//! `<session-id>.journal.jsonl` co-located with the session JSON. Each
//! line is a [`JournalEntry`] — cheap to tail, safe under concurrent
//! append, and inspectable by hand during debugging.
//!
//! ## Phase A scope
//!
//! This module delivers the type layer plus the in-memory transaction
//! state machine. The *consumers* (compaction writeback, page-invariant
//! validation hooks) come online in Phase B alongside
//! `DerivePolicy::Incremental`.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::session::journal::{JournalEntry, Op, RejectReason, WritebackJournal};
//!
//! let mut journal = WritebackJournal::new("session-42");
//! let txn = journal.stage(Op::Compaction {
//!     before: 120,
//!     after: 24,
//! });
//! journal
//!     .commit(txn)
//!     .expect("no pending validation => commit is allowed");
//! assert_eq!(journal.entries().len(), 2);
//! assert!(matches!(journal.entries()[1], JournalEntry::Committed { .. }));
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;

/// Opaque handle returned from [`WritebackJournal::stage`] and required
/// by [`WritebackJournal::validate`] / [`WritebackJournal::commit`].
///
/// Monotonic per journal instance. Crashing between stage and commit
/// leaves a `Staged` entry in the journal — the next load can reconcile
/// or abandon it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxnId(pub u64);

/// Typed operation being journalled.
///
/// Extend by adding variants rather than free-form JSON so each new
/// call site declares what it is doing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Op {
    /// Context-window enforcement rewrote the derived buffer.
    Compaction { before: usize, after: usize },
    /// `Session::save` persisted the transcript to disk.
    Save,
    /// TUI or orchestrator reset the session (e.g., `/clear`).
    Reset,
    /// Lu et al. reset-to-(prompt, summary) context reset (Phase B).
    ContextReset,
    /// MinIO history sink uploaded a delta.
    HistorySinkUpload {
        bucket: String,
        key: String,
        bytes: usize,
    },
}

/// Reason codes for a rejected writeback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum RejectReason {
    /// A destructive overwrite was requested without an explicit
    /// `allow_destructive` flag.
    DestructiveOp,
    /// Schema validation failed.
    SchemaMismatch { detail: String },
    /// Provenance / authorship check failed.
    ProvenanceFail { detail: String },
    /// The op would drop a page below its `min_fidelity` invariant.
    InvariantViolation { detail: String },
}

/// A single line in the journal.
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

/// In-memory, per-session writeback journal.
///
/// Writes go into [`Self::entries`]; a future `flush_to_disk(path)`
/// helper will stream them to `<session-id>.journal.jsonl`. Keeping
/// disk I/O out of this type makes every phase fast to unit-test.
#[derive(Debug, Clone)]
pub struct WritebackJournal {
    session_id: String,
    next_id: u64,
    pending: HashMap<TxnId, Op>,
    entries: Vec<JournalEntry>,
}

impl WritebackJournal {
    /// Create a fresh journal for `session_id`.
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            next_id: 0,
            pending: HashMap::new(),
            entries: Vec::new(),
        }
    }

    /// The session this journal belongs to.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Every entry in the order it was appended.
    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    /// Stage an op and return its [`TxnId`]. Writes a `Staged` entry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::journal::{Op, WritebackJournal};
    ///
    /// let mut journal = WritebackJournal::new("s");
    /// let txn = journal.stage(Op::Save);
    /// assert_eq!(journal.entries().len(), 1);
    /// journal.commit(txn).unwrap();
    /// assert_eq!(journal.entries().len(), 2);
    /// ```
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

    /// Mark a staged transaction as rejected with `reason`. The
    /// transaction is dropped from the pending set — a subsequent
    /// [`Self::commit`] on the same id returns `Err(RejectReason)`.
    pub fn reject(&mut self, txn: TxnId, reason: RejectReason) {
        self.pending.remove(&txn);
        self.entries.push(JournalEntry::Rejected {
            txn,
            at: Utc::now(),
            reason,
        });
    }

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

    /// Number of currently staged (neither committed nor rejected)
    /// transactions.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

/// Append journal entries to `<session-id>.journal.jsonl`.
///
/// Best-effort durability helper used by lifecycle call sites such as
/// `Session::save`. The journal remains append-only: callers hand us a
/// pre-built ordered slice and we stream it as JSONL.
pub async fn append_entries(session_id: &str, entries: &[JournalEntry]) -> anyhow::Result<()> {
    let path = crate::session::Session::sessions_dir()?.join(format!("{session_id}.journal.jsonl"));
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await?;
    for entry in entries {
        let line = serde_json::to_string(entry)?;
        file.write_all(line.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }
    file.flush().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_then_commit_appends_two_entries() {
        let mut journal = WritebackJournal::new("s1");
        let txn = journal.stage(Op::Compaction {
            before: 10,
            after: 3,
        });
        assert_eq!(journal.pending_count(), 1);
        journal.commit(txn).unwrap();
        assert_eq!(journal.pending_count(), 0);

        let entries = journal.entries();
        assert_eq!(entries.len(), 2);
        assert!(matches!(entries[0], JournalEntry::Staged { .. }));
        assert!(matches!(entries[1], JournalEntry::Committed { .. }));
    }

    #[test]
    fn rejected_writeback_does_not_commit() {
        let mut journal = WritebackJournal::new("s1");
        let txn = journal.stage(Op::Reset);
        journal.reject(txn, RejectReason::DestructiveOp);

        assert_eq!(journal.pending_count(), 0);
        let entries = journal.entries();
        assert_eq!(entries.len(), 2);
        assert!(matches!(entries[1], JournalEntry::Rejected { .. }));

        // Commit on a rejected txn is an error.
        let err = journal.commit(txn).unwrap_err();
        assert!(matches!(err, RejectReason::SchemaMismatch { .. }));
    }

    #[test]
    fn txn_ids_are_monotonic() {
        let mut journal = WritebackJournal::new("s1");
        let a = journal.stage(Op::Save);
        let b = journal.stage(Op::Save);
        assert!(b.0 > a.0);
    }

    #[test]
    fn journal_entry_round_trips_through_serde() {
        let entry = JournalEntry::Rejected {
            txn: TxnId(7),
            at: Utc::now(),
            reason: RejectReason::InvariantViolation {
                detail: "constraint below structured".into(),
            },
        };
        let json = serde_json::to_string(&entry).unwrap();
        let back: JournalEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, back);
    }
}
