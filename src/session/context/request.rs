//! Native request entry point and shared host bindings.
use super::options::RequestOptions;
use crate::provider::{CompletionRequest, Provider, ToolDefinition};
use crate::session::Session;
use anyhow::Result;
use std::sync::Arc;
#[path = "request_derive_host.rs"]
mod derive_host;
#[path = "request_host.rs"]
mod host;

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
