//! Commits an edited proposal back through the live approval gate.

#[path = "receipt.rs"]
mod receipt;

use anyhow::{Result, ensure};
use serde_json::json;

use crate::approval::{ApprovalStore, LiveApprovalDecision};
use crate::tui::app::state::approval_queue;

pub(super) fn apply(original_id: &str, patch: &str) -> Result<String> {
    if patch.trim().is_empty() {
        return reject_empty(original_id);
    }

    let revised_id = receipt::replace(original_id, patch)?;
    let decision = LiveApprovalDecision::Revised {
        arguments: json!({"patch": patch}),
        approval_id: revised_id.clone(),
    };
    let delivered = crate::approval::live::decide(original_id, decision);
    approval_queue::resolve(original_id);
    ensure!(delivered, "the waiting tool is no longer available");
    Ok(format!(
        "Edited patch approved as `{revised_id}` and sent to the waiting tool"
    ))
}

fn reject_empty(id: &str) -> Result<String> {
    let store = ApprovalStore::open_default()?;
    store.deny(
        id,
        "tui-editor",
        "all proposed changes removed in code editor",
    )?;
    crate::approval::live::decide(
        id,
        LiveApprovalDecision::denied_with("all proposed changes removed in code editor"),
    );
    approval_queue::resolve(id);
    Ok("All proposed changes removed; original tool call denied".into())
}
