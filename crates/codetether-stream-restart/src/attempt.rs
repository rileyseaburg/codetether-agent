//! One provider stream-opening and drain attempt.

use std::sync::Arc;

use tokio::sync::mpsc;

use codetether_provider_core::{CompletionRequest, Provider};
use codetether_session_events::SessionEvent;

use codetether_stream_drain::drain;
use codetether_stream_outcome::DrainOutcome;
use codetether_stream_outcome::fault_from;

pub(super) async fn run<S>(
    provider: &Arc<dyn Provider>,
    request: &CompletionRequest,
    session_id: &str,
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
) -> DrainOutcome {
    match provider
        .complete_stream_scoped(request.clone(), session_id)
        .await
    {
        Ok(stream) => drain(stream, event_tx).await,
        Err(error) => DrainOutcome {
            response: None,
            completed: Vec::new(),
            stop: fault_from(&format!("{error:#}")),
        },
    }
}
