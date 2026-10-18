//! Plain-language status text for guard-rail blocks that need no human action.
//!
//! Approval prompts are handled by `approval_hint`; this covers the other
//! structured refusals so the status bar says what happened instead of
//! showing a truncated JSON blob.

use serde_json::Value;

/// Status line for a known structured refusal, or `None` for other errors.
pub(super) fn status_text(tool: &str, output: &str) -> Option<String> {
    let value: Value = serde_json::from_str(output).ok()?;
    let code = value.get("error")?.get("code")?.as_str()?;
    let text = match code {
        "TOOL_JUSTIFICATION_REQUIRED" => format!(
            "⏸ `{tool}` held: ask mode needs a justification first. Nothing ran; the model will retry with one."
        ),
        "TOOL_APPROVAL_DENIED" => {
            format!(
                "⛔ `{tool}` denied. Nothing ran; the model was told not to repeat the same call."
            )
        }
        "FILE_EDIT_VIA_BASH_BLOCKED" => format!(
            "⛔ `{tool}` blocked: shell file writes are refused. Nothing ran; use edit/write instead."
        ),
        "LSP_PREAPPROVAL_FAILED" => format!(
            "⛔ `{tool}` blocked before approval: the proposed change has diagnostics errors."
        ),
        _ => return None,
    };
    Some(text)
}

#[cfg(test)]
#[path = "block_hint_tests.rs"]
mod tests;
