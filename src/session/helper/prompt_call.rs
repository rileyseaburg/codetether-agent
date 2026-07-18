//! Provider completion helper for one agent step.
//!
//! This module centralizes the choice between streaming and non-streaming
//! provider completion for a single agent turn. Providers are streamed only
//! when their stream path preserves structured output such as tool calls.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::{CompletionRequest, CompletionResponse, Provider};
use crate::session::SessionEvent;

#[path = "prompt_call/dispatch.rs"]
mod dispatch;

/// Hard wall-clock deadline for a single provider completion.
///
/// A stalled stream (no error, no completion) must never hang the agent loop
/// indefinitely. When this deadline elapses the step errors out loudly so the
/// caller's retry/error path can take over. Overridable via the
/// `CODETETHER_STEP_TIMEOUT_SECS` env var.
const DEFAULT_STEP_TIMEOUT_SECS: u64 = 300;

/// Execute one provider completion without bypassing retryable errors.
///
/// The helper preserves provider-level retry/error behavior by delegating
/// directly to the provider APIs instead of catching or transforming failures.
/// When tool calls are unsupported, or when the provider stream path is not
/// structured, the request is sent through [`Provider::complete`]. Otherwise,
/// the request is streamed through [`Provider::complete_stream`] and collected
/// into a single [`CompletionResponse`] while forwarding session events.
///
/// # Arguments
///
/// * `provider` - Shared provider implementation used to complete the request.
/// * `request` - Fully prepared completion request for the current agent step.
/// * `supports_tools` - Whether the active model/request path can use tool
///   calls. Requests without tool support use the non-streaming path.
/// * `event_tx` - Optional session event channel that receives streaming
///   progress while the response is collected.
///
/// # Returns
///
/// Returns the provider's completed response, including any assistant message
/// and tool-call metadata produced by the model.
///
/// # Errors
///
/// Returns an error if the provider rejects the request, the streaming response
/// cannot be opened, or the stream cannot be collected into a valid completion.
pub(super) async fn complete_step(
    provider: &Arc<dyn Provider>,
    request: CompletionRequest,
    session_id: &str,
    supports_tools: bool,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<CompletionResponse> {
    let timeout = step_timeout();
    let fut = dispatch::run(provider, request, session_id, supports_tools, event_tx);
    match tokio::time::timeout(timeout, fut).await {
        Ok(result) => result,
        Err(_) => Err(anyhow::anyhow!(
            "Provider completion exceeded the step timeout of {}s and was aborted. \
             The model stream appears to have stalled.",
            timeout.as_secs()
        )),
    }
}

/// Resolve the per-step completion deadline, honoring `CODETETHER_STEP_TIMEOUT_SECS`.
fn step_timeout() -> std::time::Duration {
    let secs = std::env::var("CODETETHER_STEP_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_STEP_TIMEOUT_SECS);
    std::time::Duration::from_secs(secs)
}
