//! Application estimators and logging for shared request-budget policy.
use anyhow::Result;
use codetether_provider_core::{CompletionRequest, Message};
use codetether_request_tokens::{estimate_request_tokens, estimate_tokens_for_messages};
/// # Arguments
/// * `request` — Fully assembled provider request to validate.
/// * `extras` — Optional context appended only if it fits.
/// # Returns
/// The validated request with fitting extras included.
/// # Errors
/// Returns a context-budget error when the required request does not fit.
/// # Examples
/// ```no_run
/// # fn example(request: codetether_provider_core::CompletionRequest) -> anyhow::Result<()> {
/// let guarded = codetether_context_request::finish(request, Vec::new())?;
/// assert!(!guarded.model.is_empty());
/// # Ok(()) }
/// ```
pub fn finish(request: CompletionRequest, extras: Vec<Message>) -> Result<CompletionRequest> {
    codetether_context_policy::request::guard::finish(
        request,
        extras,
        |messages, tools| estimate_request_tokens("", messages, tools),
        estimate_tokens_for_messages,
        |cost, used, budget| {
            tracing::debug!(target: "codetether_agent::session::context::request_guard",
                cost,
                used,
                budget,
                "Omitting optional context that exceeds input budget"
            );
        },
    )
}
