//! Morph-assisted replacement with exact matching fallback.

use serde_json::json;

use super::morph_infer::{infer_instruction, infer_update};
use codetether_tool_core::ToolResult;

pub async fn apply(
    content: &str,
    path: &str,
    old_string: Option<&str>,
    new_string: Option<&str>,
    instruction: Option<&str>,
    update: Option<&str>,
    editor: &dyn codetether_morph_backend::Editor,
) -> Result<Option<String>, ToolResult> {
    let has_pair = old_string.is_some() && new_string.is_some();
    let instruction = infer_instruction(old_string, new_string, instruction);
    let update = infer_update(old_string, new_string, update);
    match editor.apply(content, &instruction, &update).await {
        Ok(updated) => Ok(Some(updated)),
        Err(err) if has_pair => {
            tracing::warn!(target:"codetether_agent::tool::edit::morph",path = %path, error = %err, "Morph backend failed; falling back");
            Ok(None)
        }
        Err(err) => Err(ToolResult::structured_error(
            "MORPH_BACKEND_FAILED",
            "edit",
            &err.to_string(),
            None,
            Some(json!({"hint":"Configure provider credentials, or supply old_string/new_string"})),
        )),
    }
}
