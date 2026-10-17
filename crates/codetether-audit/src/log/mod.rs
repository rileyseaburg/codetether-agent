//! Bounded audit history shared across cloned handles.
//!
//! [`AuditLog`] owns private storage; child modules implement construction,
//! recording, and queries. Use its public methods through the crate-root export.

use crate::AuditEntry;
use std::{collections::VecDeque, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

mod actions;
mod construction;
mod correlation;
mod query;
mod record;

/// Maximum number of entries kept in memory by default.
const DEFAULT_MAX_ENTRIES: usize = 10_000;

/// Thread-safe, append-only audit log with bounded in-memory retention.
///
/// Clones share their history. Optional JSONL persistence is best effort and
/// does not prevent recording in memory when the sink is unavailable.
///
/// # Examples
/// ```
/// use codetether_audit::AuditLog;
/// let log = AuditLog::new(256, None);
/// let shared = log.clone();
/// assert_eq!(format!("{log:?}"), format!("{shared:?}"));
/// ```
#[derive(Clone)]
pub struct AuditLog {
    entries: Arc<RwLock<VecDeque<AuditEntry>>>,
    max_entries: usize,
    sink_path: Option<PathBuf>,
}

impl std::fmt::Debug for AuditLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuditLog")
            .field("max_entries", &self.max_entries)
            .field("sink_path", &self.sink_path)
            .finish()
    }
}
