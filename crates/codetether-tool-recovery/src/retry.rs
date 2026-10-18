//! Follow-up directive listing truncated calls and bounded retry instructions.
use codetether_provider_core::{ContentPart, Message, Role};

/// Build the follow-up user directive listing the truncated tools.
/// # Arguments
/// Receives truncated (call ID, tool name) pairs.
/// # Returns
/// A user-role retry directive naming affected tools.
/// # Examples
/// ```
/// assert_eq!(codetether_tool_recovery::retry_prompt(&[]).role, codetether_provider_core::Role::User);
/// ```
pub fn retry_prompt(tools: &[(String, String)]) -> Message {
    let names = tools
        .iter()
        .map(|(_, name)| name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let batch_rule = if tools.iter().any(|(_, name)| name == "batch") {
        " Do not use `batch` for this retry."
    } else {
        ""
    };
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!(
                "Runtime recovery directive: the previous tool call JSON was truncated ({names}). \
                 Retry now with smaller individual tool calls, at most three in this step.{batch_rule} \
                 Do not summarize or stop before attempting the smaller call."
            ),
        }],
    }
}
