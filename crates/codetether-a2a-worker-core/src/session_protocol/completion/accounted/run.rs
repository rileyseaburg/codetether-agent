use super::Host;
use codetether_provider_core::CompletionResponse;
use std::time::Instant;
/// Complete one worker step and persist its goal usage before returning.
/// # Arguments
/// `host` provides the active model, completion and accounting services.
/// # Returns
/// The provider response after durable usage accounting succeeds.
/// # Errors
/// Propagates completion failures before accounting, or accounting failures
/// before returning a successful response.
/// # Examples
/// ```no_run
/// # async fn step(host: &mut impl codetether_a2a_worker_core::session_protocol::completion::accounted::Host) -> anyhow::Result<()> {
/// let _response = codetether_a2a_worker_core::session_protocol::completion::accounted::complete(host).await?;
/// # Ok(()) }
/// ```
pub async fn complete(host: &mut impl Host) -> anyhow::Result<CompletionResponse> {
    let started = Instant::now();
    let system_prompt = host.compose();
    let temperature = Some(
        if codetether_provider_policy::prefers_temperature_one(host.model()) {
            1.0
        } else {
            0.7
        },
    );
    let response = host.complete(&system_prompt, temperature).await?;
    host.account(
        response
            .usage
            .prompt_tokens
            .saturating_add(response.usage.completion_tokens),
        started.elapsed(),
    )
    .await?;
    Ok(response)
}
