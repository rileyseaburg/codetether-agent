//! Approval branching for the patch execution pipeline.

use super::{approval, args::PatchMode, metadata, result};
use crate::approval::ApprovalReceipt;
use crate::tool::ToolResult;

pub(super) fn required(mode: &PatchMode) -> bool {
    approval::required(mode)
}

pub(super) fn pending(
    mode: &PatchMode,
    files: &[String],
    hunks: usize,
    patch: &str,
) -> Option<ToolResult> {
    if !approval::required(mode) {
        return None;
    }
    let resource = approval::resource(files);
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
    files: &[String],
) -> std::result::Result<Option<ApprovalReceipt>, ToolResult> {
    approval::verify(mode, &approval::resource(files))
}
