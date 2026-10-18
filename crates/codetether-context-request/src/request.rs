//! Construct, enrich and guard a request in the original order.
use crate::RequestOptions;
use anyhow::Result;
use codetether_provider_core::{CompletionRequest, Provider, ToolDefinition};
use std::sync::Arc;

/// Build a provider request from the session's derived context.
/// # Arguments
/// * `provider` — Primary provider used for request execution and derivation.
/// * `session` — Native preparation, derivation and recall hooks.
/// * `model` — Primary model identifier.
/// * `system_prompt` — System instructions for the request.
/// * `tools` — Available tool definitions.
/// * `opts` — Request and context-recovery options.
/// # Returns
/// A guarded provider request, including optional recall when it fits.
/// # Errors
/// Propagates preparation/derivation, budget-guard or provider failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_context_request::Host>(session: &H, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, opts: codetether_context_request::RequestOptions) -> anyhow::Result<()> {
/// let _result = codetether_context_request::build_request_with_context(provider, session, "model", "instructions", &[], opts).await?;
/// # Ok(()) }
/// ```
pub async fn build_request_with_context<H: crate::Host>(
    provider: Arc<dyn Provider>,
    session: &H,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    opts: RequestOptions,
) -> Result<CompletionRequest> {
    session.prepare(Arc::clone(&provider), model);
    let policy = opts
        .policy_override
        .unwrap_or_else(|| session.effective_policy(model));
    let derived = session
        .derive(crate::Derivation {
            provider,
            model,
            system_prompt,
            tools,
            policy,
            force_keep_last: opts.force_keep_last,
        })
        .await?;
    let prefetch = session.prefetch().await;
    let request = codetether_context_policy::request::from_derived(
        system_prompt,
        derived.messages,
        model,
        tools,
        opts,
    );
    crate::finish(request, prefetch.into_iter().collect())
}
