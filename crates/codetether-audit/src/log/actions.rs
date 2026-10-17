//! Convenience recording for actions without correlation identifiers.
//!
//! [`AuditLog::log`] constructs an [`AuditEntry`] with a fresh UUID and the
//! current UTC timestamp, then delegates recording to [`AuditLog::record`].
//! Use it when an action needs a category, outcome, and optional context but
//! does not need a duration or correlation with an OKR, relay, or session.
//! To supply those fields, construct an entry and call `record` directly.

use super::AuditLog;
use crate::{AuditCategory, AuditEntry, AuditOutcome};
use chrono::Utc;
use uuid::Uuid;

impl AuditLog {
    /// Record a simple action with a generated UUID and current UTC timestamp.
    ///
    /// See the crate-level example for recording and retrieving an action.
    ///
    /// # Arguments
    ///
    /// * `category` — Classification assigned to the audit entry.
    /// * `action` — Action name or description, converted into an owned string.
    /// * `outcome` — Result of the action being audited, not the result of
    ///   recording the entry.
    /// * `principal` — Optional identity associated with the action. `None`
    ///   leaves the principal unspecified.
    /// * `detail` — Optional structured JSON context for the action. Avoid
    ///   including credentials or other sensitive data in this payload.
    ///
    /// # Returns
    ///
    /// Returns `()` after the call to [`AuditLog::record`] finishes. This
    /// convenience method does not return the generated identifier or a
    /// recording status.
    ///
    /// # Side Effects
    ///
    /// Generates a version 4 UUID, reads the current UTC time, and submits one
    /// entry to `record`. The timestamp represents entry creation rather than
    /// a caller-supplied action time. Duration and all correlation identifiers
    /// are left unset; no principal or detail is inferred when either is absent.
    ///
    /// # Errors
    ///
    /// No error is returned to the caller. Recording behavior, including any
    /// handling of storage failures, is delegated to `record`.
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
}