//! Approval command persistence adapter.

use anyhow::Result;

use crate::approval::ApprovalStore;

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
        let receipt = store.approve(id, "tui", &action.reason)?;
        if action.intent.session_scoped() {
            crate::approval::session_grants::grant(&receipt);
            crate::approval::session_command_grants::grant_for_request(&receipt.approval_id);
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
