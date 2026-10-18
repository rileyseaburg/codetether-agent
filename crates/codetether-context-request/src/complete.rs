//! Complete guarded requests with the existing retry-option transitions.
use crate::{RequestOptions, build_request_with_context};
use anyhow::Result;
use codetether_provider_core::{CompletionResponse, Provider, ToolDefinition};
use std::sync::Arc;

/// Complete a derived-context request, prioritizing prepared RLM recovery.
/// # Arguments
/// * `provider` — Primary provider used for request execution and derivation.
/// * `session` — Native preparation, derivation and recall hooks.
/// * `model` — Primary model identifier.
/// * `system_prompt` — System instructions for the request.
/// * `tools` — Available tool definitions.
/// * `opts` — Request and context-recovery options.
/// # Returns
/// A successful provider response after any permitted context-recovery retries.
/// # Errors
/// Propagates preparation/derivation, budget-guard or provider failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_context_request::Host>(session: &H, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, opts: codetether_context_request::RequestOptions) -> anyhow::Result<()> {
/// let _result = codetether_context_request::complete_with_context(provider, session, "model", "instructions", &[], opts).await?;
/// # Ok(()) }
/// ```
pub async fn complete_with_context<H: crate::Host>(
    provider: Arc<dyn Provider>,
    session: &H,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    mut opts: RequestOptions,
) -> Result<CompletionResponse> {
    let mut attempt = 1usize;
    loop {
        let request = build_request_with_context(
            Arc::clone(&provider),
            session,
            model,
            system_prompt,
            tools,
            opts,
        )
        .await;
        let result = match request {
            Ok(request) => {
                provider
                    .complete_scoped(request, session.session_id())
                    .await
            }
            Err(error) => Err(error),
        };
        match result {
            Ok(response) => return Ok(response),
            Err(error) => {
                let Some(next) = super::recover::next(session, model, &error, attempt, opts) else {
                    return Err(error);
                };
                opts = next;
                attempt += 1;
            }
        }
    }
}
