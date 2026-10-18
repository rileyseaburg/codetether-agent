//! Native session-event binding for shared stream restart orchestration.
use crate::{
    provider::{CompletionRequest, CompletionResponse, Provider},
    session::SessionEvent,
};
pub(crate) use codetether_stream_restart::RestartPolicy;
use std::sync::Arc;
use tokio::sync::mpsc;
pub(crate) async fn run(
    provider: &Arc<dyn Provider>,
    request: CompletionRequest,
    session_id: &str,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    policy: &RestartPolicy,
) -> anyhow::Result<CompletionResponse> {
    codetether_stream_restart::run(provider, request, session_id, event_tx, policy).await
}
