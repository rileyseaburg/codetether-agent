//! Human-readable previews of tool invocations awaiting approval.
//!
//! The approval gate identifies a request by a sha256 grant key, which is
//! safe but opaque. This module derives a short, human-readable summary of
//! what the tool will actually do so an approver can make an informed
//! decision instead of approving a bare hash.

use serde_json::Value;

/// Build a short human-readable preview of a tool invocation.
///
/// Returns `None` when no meaningful preview can be derived; callers should
/// fall back to a generic reason in that case.
pub(crate) fn summarize(tool_name: &str, args: &Value) -> Option<String> {
    match tool_name {
        "bash" => field(args, "command").map(|cmd| format!("run: {}", clip(&cmd))),
        "write" => field(args, "path").map(|p| format!("write file: {p}")),
        "edit" | "multiedit" => field(args, "path").map(|p| format!("edit file: {p}")),
        "apply_patch" | "patch" => Some("apply patch to workspace".to_string()),
        _ => generic(args),
    }
}

fn generic(args: &Value) -> Option<String> {
    for key in ["path", "command", "url", "query"] {
        if let Some(value) = field(args, key) {
            return Some(format!("{key}: {}", clip(&value)));
        }
    }
    None
}

fn field(args: &Value, key: &str) -> Option<String> {
    args.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn clip(value: &str) -> String {
    let one_line = value.replace(['\n', '\r'], " ");
    if one_line.chars().count() <= 160 {
        return one_line;
    }
    let prefix: String = one_line.chars().take(157).collect();
    format!("{prefix}...")
}

#[cfg(test)]
#[path = "invocation_preview_tests.rs"]
mod tests;
