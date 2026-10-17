//! Audit event schema, including optional correlation identifiers.

use crate::{AuditCategory, AuditOutcome};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single entry in the audit log.
///
/// # Examples
/// ```
/// use codetether_audit::{AuditCategory, AuditEntry, AuditOutcome};
/// let entry = AuditEntry {
///     id: "event-1".into(), timestamp: chrono::Utc::now(),
///     category: AuditCategory::Api, action: "GET /health".into(),
///     principal: None, outcome: AuditOutcome::Success, detail: None,
///     duration_ms: None, okr_id: None, okr_run_id: None,
///     relay_id: None, session_id: None,
/// };
/// assert_eq!(entry.action, "GET /health");
/// ```
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
