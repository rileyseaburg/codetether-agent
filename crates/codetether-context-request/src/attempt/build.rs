use super::Host;
use codetether_provider_core::{CompletionRequest, Message};
/// Construct and budget a request from the current prepared attempt.
///
/// # Arguments
/// `host` supplies model settings, derived history and native preparation.
///
/// # Returns
/// A request containing system instructions and compatible derived history,
/// with optional context included when the budget permits it.
///
/// # Errors
/// Propagates request budget errors from the final guard.
///
/// # Examples
/// ```no_run
/// # async fn request(host: &mut impl codetether_context_request::attempt::Host)
/// # -> anyhow::Result<()> {
/// let request = codetether_context_request::attempt::build(host).await?;
/// assert_eq!(request.top_p, None);
/// # Ok(()) }
/// ```
pub async fn build(host: &mut impl Host) -> anyhow::Result<CompletionRequest> {
    let mut extras: Vec<Message> = host.proactive().into_iter().cloned().collect();
    host.prepare(&mut extras).await;
    let mut messages = codetether_context_policy::request::messages::build(
        host.target().system_prompt,
        host.history().to_vec(),
    );
    super::reasoning::sanitize(&mut messages, host.target().provider);
    let target = host.target();
    let request = CompletionRequest {
        messages,
        tools: target.tools.to_vec(),
        model: target.model.to_owned(),
        temperature: target.temperature,
        top_p: None,
        max_tokens: Some(
            codetether_context_policy::derivation::completion_limit::session_completion_max_tokens(
            ),
        ),
        stop: Vec::new(),
    };
    crate::finish(request, extras)
}
