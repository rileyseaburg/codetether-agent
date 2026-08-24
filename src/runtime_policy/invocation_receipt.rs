//! Fail-closed result for an invalid or consumed supplied receipt.

use crate::tool::ToolResult;

pub(super) fn rejected(tool: &str) -> ToolResult {
    ToolResult::structured_error(
        "APPROVAL_RECEIPT_REJECTED",
        tool,
        "The supplied approval receipt is invalid, mismatched, or already consumed.",
        None,
        None,
    )
}
