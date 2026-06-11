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
    ApprovalStore::open_default()
        .and_then(|store| store.create_request(TOOL, ACTION, resource, "patch write"))
        .ok()
}

pub(super) fn verify(
    mode: &PatchMode,
    resource: &str,
) -> std::result::Result<Option<ApprovalReceipt>, ToolResult> {
    if !gated(mode) {
        return Ok(None);
    }
    let Some(approval_id) = mode.approval_id.as_deref() else {
        return Ok(None);
    };
    ApprovalStore::open_default()
        .and_then(|store| store.verify(approval_id, TOOL, ACTION, resource))
        .map(Some)
        .map_err(|error| result::approval_invalid(error.to_string()))
}

pub(super) fn resource(files: &[String]) -> String {
    match files {
        [] => "workspace".to_string(),
        [file] => file.clone(),
        many => many.join(","),
    }
}

fn gated(mode: &PatchMode) -> bool {
    !mode.dry_run && env_requires_approval()
}

fn env_requires_approval() -> bool {
    std::env::var(REQUIRED_ENV).is_ok_and(|value| value == "1")
}
