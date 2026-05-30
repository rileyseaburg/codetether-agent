use super::diff;
use crate::tool::ToolResult;
use serde_json::{Value, json};

pub fn invalid(field: &str, message: &str, example: Value) -> ToolResult {
    ToolResult::structured_error(
        "INVALID_ARGUMENT",
        "edit",
        message,
        Some(vec![field]),
        Some(example),
    )
}

pub fn matcher_error(error: Value) -> ToolResult {
    let code = error["code"].as_str().unwrap_or("NOT_FOUND").to_string();
    let message = match code.as_str() {
        "AMBIGUOUS_MATCH" => "old_string matched multiple times.",
        "NOT_FOUND" => "old_string not found; closest candidate is included.",
        _ => "edit matcher failed.",
    };
    ToolResult::structured_error(&code, "edit", message, None, Some(error))
}

pub fn preview(
    path: &str,
    old: &str,
    new: &str,
    content: &str,
    updated: &str,
    backend: &str,
) -> ToolResult {
    let summary = diff::summarize(content, updated);
    if updated == content {
        return ToolResult::structured_error(
            "NO_OP",
            "edit",
            "Replacement produced no changes.",
            None,
            Some(json!({"path":path})),
        );
    }
    let mut metadata = diff::metadata(&summary);
    metadata.insert("path".into(), json!(path));
    metadata.insert("old_string".into(), json!(old));
    metadata.insert("new_string".into(), json!(new));
    metadata.insert("backend".into(), json!(backend));
    ToolResult {
        output: format!("Changes require confirmation:\n\n{}", summary.output.trim()),
        success: true,
        metadata,
    }
}
