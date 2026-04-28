//! Build provider requests from derived context.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::{CompletionRequest, ContentPart, Message, Provider, Role, ToolDefinition};
use crate::session::Session;

use super::options::RequestOptions;
use super::policy::{derive_with_policy, effective_policy};

/// Build a provider request from the session's derived context.
pub async fn build_request_with_context(
    provider: Arc<dyn Provider>,
    session: &Session,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    opts: RequestOptions,
) -> Result<CompletionRequest> {
    let policy = effective_policy(session);
    let derived = derive_with_policy(
        session,
        provider,
        model,
        system_prompt,
        tools,
        None,
        policy,
        opts.force_keep_last,
    )
    .await?;
    Ok(CompletionRequest {
        messages: request_messages(system_prompt, derived.messages),
        tools: tools.to_vec(),
        model: model.to_string(),
        temperature: opts.temperature,
        top_p: opts.top_p,
        max_tokens: opts.max_tokens,
        stop: Vec::new(),
    })
}

fn request_messages(system_prompt: &str, derived: Vec<Message>) -> Vec<Message> {
    let mut messages = vec![Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: system_prompt.to_string(),
        }],
    }];
    messages.extend(derived);
    messages
}
