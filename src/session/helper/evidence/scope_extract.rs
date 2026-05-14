use crate::provider::{ContentPart, Message, Role};

pub(crate) fn from_messages(messages: &[Message]) -> Vec<String> {
    messages
        .iter()
        .rev()
        .find(|msg| matches!(msg.role, Role::User))
        .map(text_parts)
        .map(|text| from_text(&text))
        .unwrap_or_default()
}

fn text_parts(msg: &Message) -> String {
    msg.content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn from_text(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| line.starts_with("- ") || line.starts_with("* "))
        .map(|line| line[2..].trim().to_string())
        .filter(|line| !line.is_empty())
        .take(12)
        .collect()
}
