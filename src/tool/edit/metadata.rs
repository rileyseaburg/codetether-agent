use std::collections::HashMap;

use serde_json::{Value, json};

use super::super::ToolResult;
use super::diff::DiffPreview;
use super::matcher::MatchPlan;

pub fn confirmation(
    path: &str,
    old_string: &str,
    new_string: &str,
    plan: &MatchPlan,
    preview: DiffPreview,
) -> ToolResult {
    let mut metadata: HashMap<String, Value> = HashMap::new();
    metadata.insert("requires_confirmation".to_string(), json!(true));
    metadata.insert("diff".to_string(), json!(preview.output.trim()));
    metadata.insert("added_lines".to_string(), json!(preview.added));
    metadata.insert("removed_lines".to_string(), json!(preview.removed));
    metadata.insert("path".to_string(), json!(path));
    metadata.insert(
        "old_string".to_string(),
        json!(plan.confirm_old_string(old_string)),
    );
    metadata.insert("new_string".to_string(), json!(new_string));
    metadata.insert("match_strategy".to_string(), json!(plan.strategy()));
    metadata.insert("replacements".to_string(), json!(plan.count()));
    ToolResult {
        output: format!("Changes require confirmation:\n\n{}", preview.output.trim()),
        success: true,
        metadata,
    }
}
