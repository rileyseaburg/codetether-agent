//! Scoped selection between streaming and non-streaming provider calls.

use anyhow::Result;
use codetether_provider_core::{CompletionRequest, CompletionResponse, Provider};
use codetether_session_events::SessionEvent;
use codetether_stream_restart::{RestartPolicy, run as run_with_restart};
use std::sync::Arc;
use tokio::sync::mpsc;

pub(super) async fn run<S>(
    provider: &Arc<dyn Provider>,
    request: CompletionRequest,
    session_id: &str,
    supports_tools: bool,
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
) -> Result<CompletionResponse> {
    if !supports_tools || !provider.supports_structured_streaming() {
        return provider.complete_scoped(request, session_id).await;
    }
    run_with_restart(
        provider,
        request,
        session_id,
        event_tx,
        &RestartPolicy::default(),
    )
    .await
}
