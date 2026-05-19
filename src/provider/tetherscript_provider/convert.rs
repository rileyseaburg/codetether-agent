//! Convert `CompletionRequest` messages to JSON for tetherscript.

use crate::provider::{CompletionRequest, ContentPart, Role};
use serde_json::Value;

pub fn messages(req: &CompletionRequest) -> Value {
    Value::Array(
        req.messages
            .iter()
            .map(|m| {
                let role = match m.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    Role::Tool => "tool",
                };
                let text = m
                    .content
                    .iter()
                    .filter_map(|p| {
                        if let ContentPart::Text { text } = p {
                            Some(text.as_str())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                serde_json::json!({"role": role, "content": text})
            })
            .collect(),
    )
}
