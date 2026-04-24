//! Persistent (on-disk / long-lived) telemetry façade.
//!
//! This module exposes a stable public API over what will eventually be a
//! disk-backed store. The legacy implementation returned empty collections
//! by design so that callers could be written today without waiting for the
//! storage backend; that behaviour is preserved here byte-for-byte.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::tools::{FileChange, ToolExecution};

/// Record a persistent telemetry entry. Currently a tracing-only sink;
/// the signature is kept stable so a future disk-backed implementation can
/// drop in without breaking callers.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::record_persistent;
/// use serde_json::json;
///
/// record_persistent("test", &json!({"k": "v"})).unwrap();
/// ```
pub fn record_persistent(category: &str, data: &serde_json::Value) -> Result<()> {
    tracing::debug!(category, data = ?data, "Recording persistent telemetry");
    Ok(())
}

/// Outer wrapper that matches the legacy JSON schema `{"stats": { ... }}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersistentStats {
    /// The inner stats payload.
    pub stats: PersistentStatsInner,
}

/// On-disk cumulative stats payload.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersistentStatsInner {
    /// Lifetime prompt/input tokens.
    pub total_input_tokens: u64,
    /// Lifetime completion/output tokens.
    pub total_output_tokens: u64,
    /// Lifetime provider request count.
    pub total_requests: u64,
    /// `tool_name -> invocation count`.
    pub executions_by_tool: HashMap<String, u64>,
    /// `file_path -> modification count`.
    pub files_modified: HashMap<String, u64>,
}

impl PersistentStats {
    /// Most-recent executions. Returns an empty `Vec` until the backing
    /// store is wired up (matches legacy behaviour).
    pub fn recent(&self, _limit: usize) -> Vec<ToolExecution> {
        Vec::new()
    }

    /// All recorded file changes. Returns an empty `Vec` until the backing
    /// store is wired up (matches legacy behaviour).
    pub fn all_file_changes(&self) -> Vec<(String, FileChange)> {
        Vec::new()
    }

    /// Executions filtered by tool. Returns an empty `Vec` until the backing
    /// store is wired up (matches legacy behaviour).
    pub fn by_tool(&self, _tool_name: &str) -> Vec<ToolExecution> {
        Vec::new()
    }

    /// Executions that touched a specific file. Returns an empty `Vec` until
    /// the backing store is wired up (matches legacy behaviour).
    pub fn by_file(&self, _file_path: &str) -> Vec<ToolExecution> {
        Vec::new()
    }

    /// One-line summary used by the CLI `stats` command.
    pub fn summary(&self) -> String {
        "0 total executions".to_string()
    }
}

/// Load the persistent stats. Currently returns [`PersistentStats::default`]
/// (matches legacy behaviour).
pub fn get_persistent_stats() -> PersistentStats {
    PersistentStats::default()
}
