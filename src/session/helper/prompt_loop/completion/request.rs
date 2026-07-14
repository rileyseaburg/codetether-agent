//! Construction of one provider completion request.

use super::{super::Runner, context::Attempt};
use crate::provider::{CompletionRequest, ContentPart, Message, Role};

/// Builds the provider request for the current completion attempt.
pub(super) async fn build(runner: &mut Runner<'_>, attempt: &mut Attempt) -> CompletionRequest {
    let mut messages = vec![Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: runner.model.system_prompt.clone(),
        }],
    }];
    if let Some(message) = &attempt.proactive {
        messages.push(message.clone());
    }
    super::super::super::step_model_restore::step_prepare::prepare_messages(
        &mut messages,
        runner.session,
        &mut attempt.derived.messages,
    )
    .await;
    messages.extend(attempt.derived.messages.clone());
    CompletionRequest {
        messages,
        tools: runner.model.advertised.clone(),
        model: runner.model.model_id.clone(),
        temperature: runner.model.temperature,
        top_p: None,
        max_tokens: Some(super::super::super::token::session_completion_max_tokens()),
        stop: Vec::new(),
    }
}
