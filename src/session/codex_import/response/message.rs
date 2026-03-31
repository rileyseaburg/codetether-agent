use crate::provider::{ContentPart, Message, Role};
use anyhow::Result;
use serde_json::Value;

use crate::session::codex_import::payloads::CodexResponseMessagePayload;

pub(super) fn parse_message_item(
    payload: Value,
    first_user_text: &mut Option<String>,
) -> Result<Option<Message>> {
    let payload: CodexResponseMessagePayload = serde_json::from_value(payload)?;
    let Some(role) = map_role(&payload.role) else {
        return Ok(None);
    };
    let mut content = Vec::new();

    for item in payload.content {
        match item.kind.as_str() {
            "input_text" | "output_text" => {
                let Some(text) = item.text.map(|value| value.trim().to_string()) else {
                    continue;
                };
                if text.is_empty() {
                    continue;
                }
                if role == Role::User && first_user_text.is_none() {
                    *first_user_text = Some(text.clone());
                }
                content.push(ContentPart::Text { text });
            }
            "input_image" => {
                let Some(url) = item.image_url.filter(|value| !value.is_empty()) else {
                    continue;
                };
                content.push(ContentPart::Image {
                    url,
                    mime_type: None,
                });
            }
            _ => {}
        }
    }

    Ok((!content.is_empty()).then_some(Message { role, content }))
}

fn map_role(role: &str) -> Option<Role> {
    match role {
        "user" => Some(Role::User),
        "assistant" => Some(Role::Assistant),
        _ => None,
    }
}
