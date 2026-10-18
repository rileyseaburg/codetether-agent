//! Session-context adapter for shared completion fallback policy.
use anyhow::Result;
use codetether_native_context::{ContextHost, RequestOptions};
use codetether_native_providers::{CompletionResponse, Provider, ToolDefinition};
use std::sync::Arc;
mod host;
struct ApplicationHost<'a, S> {
    provider: Arc<dyn Provider>,
    session: &'a S,
    model: &'a str,
    system_prompt: &'a str,
    options: RequestOptions,
}
/// Complete a worker step with the shared compact-schema fallback policy.
/// # Arguments
/// The provider, session, model, prompt and schemas are retained for both
/// attempts; temperature is passed unchanged and the output cap is 8192 tokens.
/// # Returns
/// The first successful completion response.
/// # Errors
/// Propagates context preparation errors or the final completion failure.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_native_context::ContextHost>(session: &S, provider: std::sync::Arc<dyn codetether_native_providers::Provider>) -> anyhow::Result<()> {
/// let _response = codetether_native_worker::completion::complete_worker_step_with_context_fallback(provider, session, "model", "system", &[], None).await?;
/// # Ok(()) }
/// ```
pub async fn complete_worker_step_with_context_fallback<S: ContextHost>(
    provider: Arc<dyn Provider>,
    session: &S,
    model: &str,
    system_prompt: &str,
    tool_definitions: &[ToolDefinition],
    temperature: Option<f32>,
) -> Result<CompletionResponse> {
    let options = RequestOptions {
        temperature,
        top_p: None,
        max_tokens: Some(8192),
        force_keep_last: None,
        policy_override: None,
    };
    codetether_a2a_worker_core::session_protocol::completion::complete_with_fallback(
        ApplicationHost {
            provider,
            session,
            model,
            system_prompt,
            options,
        },
        tool_definitions,
    )
    .await
}

/// Worker completion with native goal prompt composition and usage accounting.
pub mod accounted;
