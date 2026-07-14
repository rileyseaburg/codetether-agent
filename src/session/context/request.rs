//! Build provider requests from derived context.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::{CompletionRequest, Provider, ToolDefinition};
use crate::session::Session;

use super::options::RequestOptions;
use super::policy::{derive_with_policy, effective_policy};

#[path = "request_messages.rs"]
mod request_messages;

/// Build a provider request from the session's derived context.
pub async fn build_request_with_context(
    provider: Arc<dyn Provider>,
    session: &Session,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    opts: RequestOptions,
) -> Result<CompletionRequest> {
    crate::session::index_produce::proactive::prepare(session, Arc::clone(&provider), model);
    let policy = opts
        .policy_override
        .unwrap_or_else(|| effective_policy(session, model));
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
    let prefetch = crate::session::index::recall::prefetch::message(session).await;
    Ok(CompletionRequest {
        messages: request_messages::build(system_prompt, prefetch, derived.messages),
        tools: tools.to_vec(),
        model: model.to_string(),
        temperature: opts.temperature,
        top_p: opts.top_p,
        max_tokens: opts.max_tokens,
        stop: Vec::new(),
    })
}
