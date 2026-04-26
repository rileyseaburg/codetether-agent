//! Provider completion helper for one agent step.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::{CompletionRequest, CompletionResponse, Provider};
use crate::session::SessionEvent;

use super::stream::collect_stream_completion_with_events;

/// Execute one provider completion without bypassing retryable errors.
pub(super) async fn complete_step(
    provider: &Arc<dyn Provider>,
    request: CompletionRequest,
    supports_tools: bool,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<CompletionResponse> {
    if !supports_tools {
        return provider.complete(request).await;
    }
    let stream = provider.complete_stream(request).await?;
    collect_stream_completion_with_events(stream, event_tx).await
}
