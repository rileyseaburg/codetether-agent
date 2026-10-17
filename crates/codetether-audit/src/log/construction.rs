//! Audit log capacity and environment-based configuration.

use super::{AuditLog, DEFAULT_MAX_ENTRIES};
use std::{collections::VecDeque, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

impl Default for AuditLog {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_ENTRIES, None)
    }
}

impl AuditLog {
    /// Create an empty log, clamping `max_entries` to at least 128.
    ///
    /// `sink_path` enables best-effort JSONL appends; `None` keeps only memory.
    ///
    /// # Examples
    /// ```
    /// use codetether_audit::AuditLog;
    /// let log = AuditLog::new(256, None);
    /// assert!(format!("{log:?}").contains("256"));
    /// ```
    pub fn new(max_entries: usize, sink_path: Option<PathBuf>) -> Self {
        Self {
            entries: Arc::new(RwLock::new(VecDeque::with_capacity(
                max_entries.min(DEFAULT_MAX_ENTRIES),
            ))),
            max_entries: max_entries.max(128),
            sink_path,
        }
    }

    /// Read `CODETETHER_AUDIT_MAX_ENTRIES` and `CODETETHER_AUDIT_LOG_PATH`.
    ///
    /// Missing or invalid capacities default to 10,000. A missing path disables
    /// disk persistence. Construction does not open the file.
    ///
    /// # Examples
    /// ```
    /// let log = codetether_audit::AuditLog::from_env();
    /// assert!(format!("{log:?}").starts_with("AuditLog"));
    /// ```
    pub fn from_env() -> Self {
        let max = std::env::var("CODETETHER_AUDIT_MAX_ENTRIES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_MAX_ENTRIES);
        let path = std::env::var("CODETETHER_AUDIT_LOG_PATH")
            .ok()
            .map(PathBuf::from);
        Self::new(max, path)
    }
}
