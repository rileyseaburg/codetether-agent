//! ToolResult construction for patch outcomes.

use crate::approval::ApprovalRequest;
use crate::tool::ToolResult;
use serde_json::json;

/// Build the unified diff parse error used by the patch tool.
pub(super) fn parse_error() -> ToolResult {
    ToolResult::structured_error(
        "PARSE_ERROR",
        "apply_patch",
        "No valid hunks found in patch. Make sure the patch is in unified diff format with proper --- a/, +++ b/, and @@ headers.",
        None,
        Some(json!({
            "expected_format": "--- a/path/to/file\n+++ b/path/to/file\n@@ -start,count +start,count @@\n context line\n-removed line\n+added line\n context line",
            "hint": "Lines starting with - are removed, + are added, space are context"
        })),
    )
}

/// Build the approval-required error without modifying files.
pub(super) fn approval_required(request: Option<&ApprovalRequest>) -> ToolResult {
    let approval_id = request
        .map(|request| request.id.as_str())
        .unwrap_or("approved-change-id");
    let result = ToolResult::structured_error(
        "PATCH_APPROVAL_REQUIRED",
        "apply_patch",
        "Patch approval is required before modifying files. Retry with approval_id to apply.",
        Some(vec!["approval_id"]),
        Some(json!({"approval_id": approval_id})),
    );
    match request {
        Some(request) => result.with_metadata("approval_request_id", json!(request.id.clone())),
        None => result,
    }
}

/// Build an invalid approval error without modifying files.
pub(super) fn approval_invalid(reason: String) -> ToolResult {
    ToolResult::structured_error(
        "PATCH_APPROVAL_INVALID",
        "apply_patch",
        "Patch approval_id was not approved for this patch.",
        Some(vec!["approval_id"]),
        Some(json!({"reason": reason})),
    )
}
