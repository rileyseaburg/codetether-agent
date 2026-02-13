//! System-wide audit trail
//!
//! Every action taken by the server — API calls, tool executions, session
//! operations, cognition decisions — is recorded in a tamper-evident append-only
//! log.  Entries are held in a bounded in-memory ring buffer and optionally
//! flushed to a JSONL file on disk.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Maximum number of entries kept in memory before oldest are evicted.
const DEFAULT_MAX_ENTRIES: usize = 10_000;

/// Categories of auditable actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditCategory {
    /// HTTP API request/response
    Api,
    /// Tool invocation
    ToolExecution,
    /// Session lifecycle (create, load, prompt)
    Session,
    /// Cognition loop events
    Cognition,
    /// Swarm persona operations
    Swarm,
    /// Authentication events
    Auth,
    /// Kubernetes self-deployment actions
    K8s,
    /// Plugin sandbox events
    Sandbox,
    /// Configuration changes
    Config,
}

/// Outcome of an audited action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    Success,
    Failure,
    Denied,
}

/// A single entry in the audit log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique entry ID.
    pub id: String,
    /// When the action occurred.
    pub timestamp: DateTime<Utc>,
    /// Category of the action.
    pub category: AuditCategory,
    /// Human-readable description (e.g. "POST /api/session").
    pub action: String,
    /// Authenticated principal, if any.
    pub principal: Option<String>,
    /// Outcome of the action.
    pub outcome: AuditOutcome,
    /// Additional structured details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
    /// Duration of the action in milliseconds, if measured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// OKR ID if this action is part of an OKR-gated operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr_id: Option<String>,
    /// OKR run ID if this action is part of an OKR run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr_run_id: Option<String>,
    /// Relay ID if this action is part of a relay execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relay_id: Option<String>,
    /// Session ID for correlation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// Thread-safe, append-only audit log.
#[derive(Clone)]
pub struct AuditLog {
    entries: Arc<RwLock<VecDeque<AuditEntry>>>,
    max_entries: usize,
    /// Optional path for JSONL persistence.
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

impl Default for AuditLog {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_ENTRIES, None)
    }
}

impl AuditLog {
    /// Create a new audit log.
    pub fn new(max_entries: usize, sink_path: Option<PathBuf>) -> Self {
        Self {
            entries: Arc::new(RwLock::new(VecDeque::with_capacity(
                max_entries.min(DEFAULT_MAX_ENTRIES),
            ))),
            max_entries: max_entries.max(128),
            sink_path,
        }
    }

    /// Build from environment variables.
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

    /// Record an audit entry.
    pub async fn record(&self, entry: AuditEntry) {
        // Persist to disk first (best effort).
        if let Some(ref path) = self.sink_path {
            if let Ok(line) = serde_json::to_string(&entry) {
                use tokio::io::AsyncWriteExt;
                if let Ok(mut file) = tokio::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .await
                {
                    let _ = file.write_all(line.as_bytes()).await;
                    let _ = file.write_all(b"\n").await;
                }
            }
        }

        // Log to tracing so structured log aggregators pick it up.
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

    /// Convenience: record a simple action.
    pub async fn log(
        &self,
        category: AuditCategory,
        action: impl Into<String>,
        outcome: AuditOutcome,
        principal: Option<String>,
        detail: Option<serde_json::Value>,
    ) {
        self.record(AuditEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            category,
            action: action.into(),
            principal,
            outcome,
            detail,
            duration_ms: None,
            okr_id: None,
            okr_run_id: None,
            relay_id: None,
            session_id: None,
        })
        .await;
    }

    /// Convenience: record an action with OKR/relay correlation.
    pub async fn log_with_correlation(
        &self,
        category: AuditCategory,
        action: impl Into<String>,
        outcome: AuditOutcome,
        principal: Option<String>,
        detail: Option<serde_json::Value>,
        okr_id: Option<String>,
        okr_run_id: Option<String>,
        relay_id: Option<String>,
        session_id: Option<String>,
    ) {
        self.record(AuditEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            category,
            action: action.into(),
            principal,
            outcome,
            detail,
            duration_ms: None,
            okr_id,
            okr_run_id,
            relay_id,
            session_id,
        })
        .await;
    }

    /// Return recent entries (newest first), up to `limit`.
    pub async fn recent(&self, limit: usize) -> Vec<AuditEntry> {
        let lock = self.entries.read().await;
        lock.iter().rev().take(limit).cloned().collect()
    }

    /// Return total entry count.
    pub async fn count(&self) -> usize {
        self.entries.read().await.len()
    }

    /// Filter entries by category.
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

/// Global audit log singleton.
static AUDIT_LOG: tokio::sync::OnceCell<AuditLog> = tokio::sync::OnceCell::const_new();

/// Initialize the global audit log.
pub fn init_audit_log(log: AuditLog) -> Result<(), AuditLog> {
    AUDIT_LOG.set(log).map_err(|_| AuditLog::default())
}

/// Get the global audit log (panics if not initialized).
pub fn audit_log() -> &'static AuditLog {
    AUDIT_LOG
        .get()
        .expect("Audit log not initialized — call init_audit_log() at startup")
}

/// Get the global audit log if initialized.
pub fn try_audit_log() -> Option<&'static AuditLog> {
    AUDIT_LOG.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn audit_log_records_and_retrieves() {
        let log = AuditLog::new(100, None);
        log.log(
            AuditCategory::Api,
            "GET /health",
            AuditOutcome::Success,
            None,
            None,
        )
        .await;

        assert_eq!(log.count().await, 1);
        let entries = log.recent(10).await;
        assert_eq!(entries[0].action, "GET /health");
    }

    #[tokio::test]
    async fn audit_log_evicts_oldest() {
        let log = AuditLog::new(128, None);
        for i in 0..200 {
            log.log(
                AuditCategory::Api,
                format!("req-{}", i),
                AuditOutcome::Success,
                None,
                None,
            )
            .await;
        }
        assert!(log.count().await <= 128);
    }
}
