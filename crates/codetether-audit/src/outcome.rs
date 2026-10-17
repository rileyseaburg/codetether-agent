//! Outcomes shared by all audit event categories.

use serde::{Deserialize, Serialize};

/// Outcome of an audited action, serialized as a snake_case string.
///
/// # Examples
/// ```
/// use codetether_audit::AuditOutcome;
/// let label = match AuditOutcome::Denied {
///     AuditOutcome::Success => "succeeded",
///     AuditOutcome::Failure => "failed",
///     AuditOutcome::Denied => "denied",
/// };
/// assert_eq!(label, "denied");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    /// The action succeeded.
    Success,
    /// The action failed during execution.
    Failure,
    /// The action was denied before execution.
    Denied,
}
