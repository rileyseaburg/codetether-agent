//! Approval command persistence adapter.

use anyhow::Result;

use crate::approval::{ApprovalDecisionKind, ApprovalStore};

pub(super) struct StoredDecision {
    pub(super) id: String,
    pub(super) tool: Option<String>,
}

pub(super) fn record(
    store: &ApprovalStore,
    id: &str,
    action: &super::parse::Action<'_>,
) -> Result<StoredDecision> {
    if action.intent.approves() {
        let kind = if action.intent.session_scoped() {
            ApprovalDecisionKind::ApproveForSession
        } else {
            ApprovalDecisionKind::ApproveOnce
        };
        let receipt = store.approve_review(id, "tui", &action.reason, kind)?;
        kind.grant_session(&receipt);
        return Ok(StoredDecision {
            id: receipt.approval_id,
            tool: Some(receipt.tool),
        });
    }
    store.deny(id, "tui", &action.reason)?;
    crate::approval::session_settle::request(id, None);
    Ok(StoredDecision {
        id: id.to_string(),
        tool: None,
    })
}
