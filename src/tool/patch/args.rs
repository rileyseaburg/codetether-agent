//! Request argument parsing for the patch tool.

use crate::tool::ToolResult;
use serde_json::{Value, json};

/// Execution mode parsed from patch request flags.
pub(super) struct PatchMode {
    pub dry_run: bool,
    pub approval_id: Option<String>,
}

/// Parse mode flags that are useful even when the patch body is invalid.
pub(super) fn mode(params: &Value) -> PatchMode {
    PatchMode {
        dry_run: bool_param(params, "dry_run") || bool_param(params, "preview"),
        approval_id: params
            .get("approval_id")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string),
    }
}

/// Extract the required unified diff body.
pub(super) fn patch(params: &Value) -> Result<String, ToolResult> {
    match params.get("patch").and_then(Value::as_str) {
        Some(patch) if !patch.is_empty() => Ok(patch.to_string()),
        _ => Err(ToolResult::structured_error(
            "MISSING_FIELD",
            "apply_patch",
            "patch is required and must be a non-empty string containing a unified diff",
            Some(vec!["patch"]),
            Some(json!({
                "patch": "--- a/file.rs\n+++ b/file.rs\n@@ -1,3 +1,3 @@\n line1\n-old line\n+new line\n line3",
                "dry_run": false,
                "preview": false,
                "approval_id": "optional-approval-token"
            })),
        )),
    }
}

fn bool_param(params: &Value, key: &str) -> bool {
    params.get(key).and_then(Value::as_bool).unwrap_or(false)
}
