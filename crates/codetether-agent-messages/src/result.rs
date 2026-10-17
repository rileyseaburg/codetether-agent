//! Structured result rendering for child-agent message exchanges.
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
/// Render a child response with optional reasoning, tool previews and failure.
/// # Arguments
/// * `agent_id` — Durable child identity.
/// * `name` — Display name.
/// * `response` — Response text, also used as serialization fallback.
/// * `thinking` — Optional nonempty reasoning text.
/// * `tools` — Tool previews, omitted when empty.
/// * `error` — Runtime or deliverable warning, marking the result unsuccessful.
/// # Returns
/// A JSON tool result retaining response text even on failure.
/// # Examples
/// ```rust
/// let result = codetether_agent_messages::build_message_result(
///     "child".into(), "reviewer".into(), "done".into(), String::new(), vec![], None,
/// );
/// assert!(result.success);
/// ```
pub fn build_message_result(
    agent_id: String,
    name: String,
    response: String,
    thinking: String,
    tools: Vec<Value>,
    error: Option<String>,
) -> ToolResult {
    let fallback = response.clone();
    let mut output = json!({ "agent_id": agent_id, "agent": name, "response": response });
    if !thinking.is_empty() {
        output["thinking"] = json!(thinking);
    }
    if !tools.is_empty() {
        output["tool_calls"] = json!(tools);
    }
    if let Some(error) = error {
        output["warning"] = json!(error);
        return ToolResult::error(serde_json::to_string_pretty(&output).unwrap_or(fallback));
    }
    ToolResult::success(serde_json::to_string_pretty(&output).unwrap_or(fallback))
}
