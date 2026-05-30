use serde_json::json;

use super::super::{ToolResult, morph_backend};

pub async fn apply(
    content: &str,
    path: &str,
    old_string: Option<&str>,
    new_string: Option<&str>,
    instruction: Option<&str>,
    update: Option<&str>,
) -> Result<Option<String>, ToolResult> {
    let has_pair = old_string.is_some() && new_string.is_some();
    let instruction = infer_instruction(old_string, new_string, instruction);
    let update = infer_update(old_string, new_string, update);
    match morph_backend::apply_edit_with_morph(content, &instruction, &update).await {
        Ok(updated) => Ok(Some(updated)),
        Err(err) if has_pair => {
            tracing::warn!(path = %path, error = %err, "Morph backend failed; falling back");
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

fn infer_instruction(old: Option<&str>, new: Option<&str>, instruction: Option<&str>) -> String {
    instruction.map(str::to_string).or_else(|| old.zip(new).map(|(old, new)| {
        format!("Replace the target snippet exactly once.\nOld snippet:\n{old}\n\nNew snippet:\n{new}")
    })).unwrap_or_else(|| "Apply the requested update precisely.".to_string())
}

fn infer_update(old: Option<&str>, new: Option<&str>, update: Option<&str>) -> String {
    update.map(str::to_string).or_else(|| old.zip(new).map(|(old, new)| {
        format!("// Replace this snippet:\n{old}\n// With this snippet:\n{new}\n// ...existing code...")
    })).unwrap_or_else(|| "// ...existing code...".to_string())
}
