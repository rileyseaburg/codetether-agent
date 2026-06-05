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

use super::stream::collect_stream_completion_with_events;

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
    supports_tools: bool,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<CompletionResponse> {
    if !supports_tools || !provider.supports_structured_streaming() {
        return provider.complete(request).await;
    }
    let stream = provider.complete_stream(request).await?;
    collect_stream_completion_with_events(stream, event_tx).await
}
