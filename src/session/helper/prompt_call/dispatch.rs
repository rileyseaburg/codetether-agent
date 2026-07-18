//! Scoped selection between streaming and non-streaming provider calls.

use super::super::stream::{RestartPolicy, run_with_restart};
use crate::provider::{CompletionRequest, CompletionResponse, Provider};
use crate::session::SessionEvent;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

pub(super) async fn run(
    provider: &Arc<dyn Provider>,
    request: CompletionRequest,
    session_id: &str,
    supports_tools: bool,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
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
