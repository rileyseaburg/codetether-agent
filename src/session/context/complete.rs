//! Native completion entry point backed by shared retry orchestration.
use super::options::RequestOptions;
use crate::provider::{CompletionResponse, Provider, ToolDefinition};
use crate::session::Session;
use anyhow::Result;
use std::sync::Arc;

/// Complete a derived-context request, prioritizing prepared RLM recovery.
pub async fn complete_with_context(
    provider: Arc<dyn Provider>,
    session: &Session,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    opts: RequestOptions,
) -> Result<CompletionResponse> {
    codetether_context_request::complete_with_context(
        provider,
        session,
        model,
        system_prompt,
        tools,
        opts,
    )
    .await
}
