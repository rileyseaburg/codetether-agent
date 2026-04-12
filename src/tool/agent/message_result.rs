//! JSON result rendering for sub-agent messages.
//!
//! This module formats the streamed response, thinking trace, and tool preview
//! into the final `ToolResult`.
//!
//! # Examples
//!
//! ```ignore
//! let result = build_message_result("name".into(), "ok".into(), String::new(), vec![], None);
//! ```

use crate::tool::ToolResult;
use serde_json::{Value, json};

/// Builds the final tool result for a sub-agent message exchange.
///
/// # Examples
///
/// ```ignore
/// let result = build_message_result("name".into(), "ok".into(), String::new(), vec![], None);
/// ```
pub(super) fn build_message_result(
    name: String,
    response: String,
    thinking: String,
    tools: Vec<Value>,
    error: Option<String>,
) -> ToolResult {
    let fallback = response.clone();
    let mut output = json!({ "agent": name, "response": response });
    if !thinking.is_empty() {
        output["thinking"] = json!(thinking);
    }
    if !tools.is_empty() {
        output["tool_calls"] = json!(tools);
    }
    if let Some(error) = error {
        if fallback.is_empty() {
            return ToolResult::error(format!(
                "Agent @{} failed: {error}",
                output["agent"].as_str().unwrap_or("unknown")
            ));
        }
        output["warning"] = json!(error);
    }
    ToolResult::success(serde_json::to_string_pretty(&output).unwrap_or(fallback))
}
