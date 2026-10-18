//! Audit-list query parameters.
use serde::Deserialize;

// ── Audit trail endpoints ──

#[derive(Deserialize)]
/// Optional history limit and category name for audit queries.
/// # Examples
/// ```
/// let query = codetether_audit_http::AuditQuery { limit: Some(10), category: Some("api".into()) };
/// assert_eq!(query.limit, Some(10));
/// ```
pub struct AuditQuery {
    /// Requested maximum number of entries.
    pub limit: Option<usize>,
    /// Optional category name or supported alias.
    pub category: Option<String>,
}
