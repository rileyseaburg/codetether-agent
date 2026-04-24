//! Build provider requests for the autochat relay.

use crate::provider::{CompletionRequest, ContentPart, Message, Role};

/// Create the provider completion request for an autochat task.
pub fn build_request(task: String, model: String) -> CompletionRequest {
    CompletionRequest {
        model,
        messages: vec![system_message(), user_message(task)],
        tools: Vec::new(),
        temperature: Some(1.0),
        top_p: Some(0.9),
        max_tokens: Some(1200),
        stop: Vec::new(),
    }
}

fn system_message() -> Message {
    Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: prompt().to_string(),
        }],
    }
}

fn user_message(task: String) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: task }],
    }
}

fn prompt() -> &'static str {
    "You are the CodeTether autochat relay. Execute the task directly and return a concise completion summary with concrete actions taken or next steps."
}
