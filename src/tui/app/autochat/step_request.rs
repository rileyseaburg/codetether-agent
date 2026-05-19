//! Provider request builder for a single relay step.

use crate::provider::{CompletionRequest, ContentPart, Message, Role};

use super::persona::Persona;

/// Build the request for one persona turn.
///
/// The relay feeds the running baton (previous output) forward as
/// extra user-message context so each persona can build on it.
pub fn build_step_request(
    model: String,
    persona: &Persona,
    task: &str,
    baton: &str,
) -> CompletionRequest {
    CompletionRequest {
        model,
        messages: vec![system_message(persona), user_message(task, baton)],
        tools: Vec::new(),
        temperature: Some(0.9),
        top_p: Some(0.9),
        max_tokens: Some(1000),
        stop: Vec::new(),
    }
}

fn system_message(persona: &Persona) -> Message {
    Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: persona.instructions.to_string(),
        }],
    }
}

fn user_message(task: &str, baton: &str) -> Message {
    let mut text = format!("Task:\n{task}\n");
    if !baton.trim().is_empty() {
        text.push_str("\nPrevious relay output:\n");
        text.push_str(baton);
    }
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text }],
    }
}
