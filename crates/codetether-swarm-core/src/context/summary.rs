use codetether_provider_core::{ContentPart, Message};

/// Summarize unique tool names in the order first encountered.
///
/// # Arguments
/// * `messages` - Removed history to summarize.
///
/// # Returns
/// A tool-use summary, or an empty string if there were no tool calls.
///
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_core::context::summarize_removed_messages(&[]), "");
/// ```
pub fn summarize_removed_messages(messages: &[Message]) -> String {
    let mut tool_calls: Vec<String> = Vec::new();
    for msg in messages {
        for part in &msg.content {
            if let ContentPart::ToolCall { name, .. } = part {
                push_unique(&mut tool_calls, name);
            }
        }
    }
    if tool_calls.is_empty() {
        String::new()
    } else {
        format!("Tools used in truncated history: {}", tool_calls.join(", "))
    }
}

fn push_unique(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|seen| seen == value) {
        values.push(value.to_string());
    }
}
