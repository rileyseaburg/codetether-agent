//! Native request entry point and shared host bindings.
use super::options::RequestOptions;
use crate::provider::{CompletionRequest, Provider, ToolDefinition};
use crate::session::Session;
use anyhow::Result;
use std::sync::Arc;
codetether_context_request::impl_session_context!(
    Session,
    super::policy::effective_policy,
    crate::session::index_produce::proactive::prepare,
    super::policy::derive_with_policy,
    crate::session::index::recall::prefetch::message
);

/// Build a provider request from the session's derived context.
pub async fn build_request_with_context(
    provider: Arc<dyn Provider>,
    session: &Session,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    opts: RequestOptions,
) -> Result<CompletionRequest> {
    codetether_context_request::build_request_with_context(
        provider,
        session,
        model,
        system_prompt,
        tools,
        opts,
    )
    .await
}
