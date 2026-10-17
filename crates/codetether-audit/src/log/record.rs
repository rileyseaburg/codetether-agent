//! Recording pipeline: persist, emit tracing, then retain in bounded history.

use super::AuditLog;
use crate::{AuditEntry, sink};

impl AuditLog {
    /// Record an entry unchanged, evicting the oldest entries at capacity.
    ///
    /// Disk writes are attempted first; failures do not prevent tracing or
    /// in-memory retention. See [`Self::log`] for generated IDs and timestamps.
    pub async fn record(&self, entry: AuditEntry) {
        if let Some(ref path) = self.sink_path {
            sink::append(path, &entry).await;
        }
        tracing::info!(
            audit_id = %entry.id,
            category = ?entry.category,
            action = %entry.action,
            outcome = ?entry.outcome,
            principal = entry.principal.as_deref().unwrap_or("-"),
            "audit"
        );
        let mut lock = self.entries.write().await;
        lock.push_back(entry);
        while lock.len() > self.max_entries {
            lock.pop_front();
        }
    }
}
