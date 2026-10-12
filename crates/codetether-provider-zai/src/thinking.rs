//! Thinking policy for exact answers and short, low-budget requests.
use crate::{CompletionRequest, ContentPart, Message};
use serde_json::{Value, json};

pub(crate) fn should_disable_thinking_for_request(request: &CompletionRequest) -> bool {
    request
        .messages
        .iter()
        .any(contains_explicit_short_answer_instruction)
        || (request.max_tokens.is_some_and(|max| max <= 256)
            && request.messages.iter().any(|message| {
                message.content.iter().any(|part| {
                    matches!(part,
                    ContentPart::Text { text } if text.chars().count() <= 512)
                })
            }))
}

fn contains_explicit_short_answer_instruction(message: &Message) -> bool {
    message.content.iter().any(|part| {
        if let ContentPart::Text { text } = part {
            let lower = text.to_ascii_lowercase();
            [
                "reply with exactly",
                "say only",
                "respond with exactly",
                "return exactly",
            ]
            .iter()
            .any(|phrase| lower.contains(phrase))
        } else {
            false
        }
    })
}

pub(crate) fn thinking_config_for_request(request: &CompletionRequest) -> Value {
    if should_disable_thinking_for_request(request) {
        json!({"type": "disabled"})
    } else {
        json!({"type": "enabled", "clear_thinking": true})
    }
}
