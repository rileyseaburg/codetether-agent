//! Approval policy for mutating patch execution.

use super::{args::PatchMode, result};
use crate::approval::{ApprovalReceipt, ApprovalRequest, ApprovalStore};
use crate::tool::ToolResult;

const REQUIRED_ENV: &str = "CODETETHER_PATCH_APPROVAL_REQUIRED";
const TOOL: &str = "apply_patch";
const ACTION: &str = "write";

/// Return whether this patch needs an external approval id before writing.
pub(super) fn required(mode: &PatchMode) -> bool {
    gated(mode) && mode.approval_id.is_none()
}

pub(super) fn request(resource: &str) -> Option<ApprovalRequest> {
    let tool = crate::tool::alias::policy_id(TOOL);
    ApprovalStore::open_default()
        .and_then(|store| store.create_request(&tool, ACTION, resource, "patch write"))
        .ok()
}

pub(super) fn verify(
    mode: &PatchMode,
    resource: &str,
) -> std::result::Result<Option<ApprovalReceipt>, ToolResult> {
    let Some(approval_id) = mode.approval_id.as_deref() else {
        return Ok(None);
    };
    ApprovalStore::open_default()
        .and_then(|store| {
            let request = store
                .request(approval_id)?
                .ok_or_else(|| anyhow::anyhow!("approval request not found"))?;
            if !matches!(request.tool.as_str(), TOOL | "patch") {
                anyhow::bail!("approval tool mismatch");
            }
            store.claim(
                approval_id,
                &request.tool,
                ACTION,
                resource,
                "apply-patch-runtime",
            )
        })
        .map(Some)
        .map_err(|error| result::approval_invalid(error.to_string()))
}

fn gated(mode: &PatchMode) -> bool {
    !mode.dry_run && env_requires_approval()
}

fn env_requires_approval() -> bool {
    std::env::var(REQUIRED_ENV).is_ok_and(|value| value == "1")
}
