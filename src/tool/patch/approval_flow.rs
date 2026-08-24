//! Approval branching for the patch execution pipeline.

use super::{approval, approval_scope, args::PatchMode, metadata, result};
use crate::approval::ApprovalReceipt;
use crate::tool::ToolResult;

pub(super) fn required(mode: &PatchMode) -> bool {
    approval::required(mode)
}

pub(super) fn pending(
    mode: &PatchMode,
    root: &std::path::Path,
    files: &[String],
    hunks: usize,
    patch: &str,
) -> Option<ToolResult> {
    if !approval::required(mode) {
        return None;
    }
    let resource = approval_scope::for_apply(root, files, patch);
    let request = approval::request(&resource);
    Some(metadata::attach(
        result::approval_required(request.as_ref()),
        files,
        hunks,
        patch,
        true,
    ))
}

pub(super) fn verify(
    mode: &PatchMode,
    root: &std::path::Path,
    files: &[String],
    patch: &str,
) -> std::result::Result<Option<ApprovalReceipt>, ToolResult> {
    let resource = approval_scope::for_mode(root, files, patch, mode.dry_run);
    approval::verify(mode, &resource)
}
