//! Recovery messaging for tool calls truncated by the provider's output cap.
//!
//! When a model's response hits the output-token limit mid tool-call, the
//! arguments JSON cannot be parsed. These helpers build concrete, tool-aware
//! corrective messages so the model retries with a smaller call instead of
//! re-truncating the same oversized one.

mod rules;
#[cfg(test)]
mod tests;

use crate::provider::{ContentPart, Message, Role};

use rules::retry_rule;

/// Build the tool-result error text shown to the model for a truncated call.
pub(in crate::session::helper) fn error_content(tool: &str) -> String {
    format!(
        "Error: Your tool call to `{tool}` was truncated because the provider cut off the \
         arguments JSON before it could be parsed (the response hit the output-token limit). \
         Recovery required: {} Do not \
         provide a final answer until the smaller tool call succeeds or you have tried a \
         different small diagnostic step.",
        retry_rule(tool)
    )
}

/// Build the follow-up user directive listing the truncated tools.
pub(in crate::session::helper) fn retry_prompt(tools: &[(String, String)]) -> Message {
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
