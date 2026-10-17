//! Action construction with OKR, relay, and session correlation.

use super::AuditLog;
use crate::{AuditCategory, AuditEntry, AuditOutcome};
use chrono::Utc;
use uuid::Uuid;

impl AuditLog {
    /// Record an action with optional OKR, run, relay, and session IDs.
    ///
    /// The entry gets a fresh UUID and UTC timestamp; duration is left unset.
    ///
    /// # Examples
    /// ```
    /// use codetether_audit::{AuditCategory, AuditLog, AuditOutcome};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let log = AuditLog::default();
    /// log.log_with_correlation(AuditCategory::Session, "create", AuditOutcome::Success,
    ///     None, None, None, None, None, Some("session-1".into())).await;
    /// assert_eq!(log.recent(1).await[0].session_id.as_deref(), Some("session-1"));
    /// # });
    /// ```
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
}
