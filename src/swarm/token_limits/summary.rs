use crate::provider::{ContentPart, Message};

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
