//! Approval-store receipts for a user-edited patch proposal.

use anyhow::Result;

use codetether_approval_core::ApprovalStore;

pub(super) fn replace(original_id: &str, patch: &str) -> Result<String> {
    let store = ApprovalStore::open_default()?;
    let resource = codetether_patch::approval_resource_from_patch(patch);
    let revised = store.create_request(
        "apply_patch",
        "write",
        &resource,
        "user-edited patch proposal",
    )?;
    store.approve(
        &revised.id,
        "tui-editor",
        "edited and approved in code editor",
    )?;
    store.deny(original_id, "tui-editor", "superseded by edited patch")?;
    Ok(revised.id)
}
