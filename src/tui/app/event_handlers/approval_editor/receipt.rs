//! Approval-store receipts for a user-edited patch proposal.

use anyhow::Result;
use std::path::Path;

use crate::approval::ApprovalStore;

pub(super) fn replace(original_id: &str, patch: &str) -> Result<String> {
    let store = ApprovalStore::open_default()?;
    let original = store
        .request(original_id)?
        .ok_or_else(|| anyhow::anyhow!("original approval request not found"))?;
    let root = original
        .resource
        .strip_prefix("root=")
        .and_then(|value| value.split("::").next())
        .ok_or_else(|| anyhow::anyhow!("original patch approval has no workspace scope"))?;
    let resource = crate::tool::patch::approval_resource_for_root(Path::new(root), patch);
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
    if let Err(error) = store.deny(original_id, "tui-editor", "superseded by edited patch") {
        store.consume(&revised.id, "tui-editor")?;
        return Err(error);
    }
    crate::approval::session_settle::request(original_id, None);
    Ok(revised.id)
}

pub(super) fn revoke(revised_id: &str) -> Result<()> {
    ApprovalStore::open_default()?.consume(revised_id, "tui-editor")
}
