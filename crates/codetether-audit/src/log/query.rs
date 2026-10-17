//! Read-only snapshots of the retained in-memory history.

use super::AuditLog;
use crate::{AuditCategory, AuditEntry};

impl AuditLog {
    /// Return recent entries (newest first), up to `limit`; zero returns none.
    pub async fn recent(&self, limit: usize) -> Vec<AuditEntry> {
        let lock = self.entries.read().await;
        lock.iter().rev().take(limit).cloned().collect()
    }

    /// Return the number of entries currently retained in memory.
    pub async fn count(&self) -> usize {
        self.entries.read().await.len()
    }

    /// Return up to `limit` entries matching `category`, newest first.
    pub async fn by_category(&self, category: AuditCategory, limit: usize) -> Vec<AuditEntry> {
        let lock = self.entries.read().await;
        lock.iter()
            .rev()
            .filter(|e| e.category == category)
            .take(limit)
            .cloned()
            .collect()
    }
}
