//! Build a synthetic call only when a registered bash tool and command are present.
use codetether_provider_core::ToolDefinition;

/// Attempt to build a synthetic `(id, name, arguments)` tool call from prose.
///
/// Returns `None` when no high-confidence extraction is possible. Specializes
/// in `bash` (the most common narrated action); other tools are deferred to the
/// retry path to avoid fabricating arguments for schemas we cannot infer.
/// # Arguments
/// Receives assistant text and the registered tools.
/// # Returns
/// A generated call ID, bash tool name and arguments when extraction succeeds.
/// # Examples
/// ```
/// assert!(codetether_tool_recovery::extract_tool_call_from_prose("run `ls`", &[]).is_none());
/// ```
pub fn extract_tool_call_from_prose(
    text: &str,
    tool_definitions: &[ToolDefinition],
) -> Option<(String, String, serde_json::Value)> {
    let lower = text.to_ascii_lowercase();
    let has_bash = tool_definitions.iter().any(|t| t.name == "bash");
    if !has_bash || !lower.contains("bash") {
        return None;
    }
    let cmd = crate::extract_bash_command(text)?;
    let id = format!("synthetic_{}", &uuid::Uuid::new_v4().to_string()[..8]);
    Some((
        id,
        "bash".to_string(),
        serde_json::json!({ "command": cmd }),
    ))
}
