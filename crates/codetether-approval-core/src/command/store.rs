//! Approval command persistence adapter.

use anyhow::Result;

use crate::ApprovalStore;

/// Identity and optional tool name returned after recording a decision.
///
/// # Examples
/// ```
/// let result = codetether_approval_core::command::StoredDecision { id: "request-1".into(), tool: None };
/// assert!(result.tool.is_none());
/// ```
pub struct StoredDecision {
    /// ID of the recorded approval request.
    pub id: String,
    /// Tool name from an approved receipt, absent for denials.
    pub tool: Option<String>,
}

/// Record a resolved request ID using the command's original grant semantics.
///
/// # Arguments
/// * `store` — Approval ledger to update.
/// * `id` — Request ID selected by the host.
/// * `action` — Parsed intent and reason.
///
/// # Returns
/// Recorded ID and approved tool metadata.
///
/// # Errors
/// Propagates ledger approval or denial errors.
///
/// # Panics
/// Session grants may panic if their shared mutexes were poisoned.
///
/// # Examples
/// ```no_run
/// # fn apply(store: &codetether_approval_core::ApprovalStore) -> anyhow::Result<()> {
/// let action = codetether_approval_core::command::Action::parse("/deny request-1").unwrap();
/// let _stored = codetether_approval_core::command::record(store, "request-1", &action)?;
/// # Ok(()) }
/// ```
pub fn record(
    store: &ApprovalStore,
    id: &str,
    action: &super::parse::Action<'_>,
) -> Result<StoredDecision> {
    if action.intent.approves() {
        let receipt = store.approve(id, "tui", &action.reason)?;
        if action.intent.session_scoped() {
            crate::session_grants::grant(&receipt);
            crate::session_command_grants::grant_for_request(&receipt.approval_id);
        }
        return Ok(StoredDecision {
            id: receipt.approval_id,
            tool: Some(receipt.tool),
        });
    }
    store.deny(id, "tui", &action.reason)?;
    Ok(StoredDecision {
        id: id.to_string(),
        tool: None,
    })
}
