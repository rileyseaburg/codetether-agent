//! One provider stream-opening and drain attempt.

use std::sync::Arc;

use tokio::sync::mpsc;

use crate::provider::{CompletionRequest, Provider};
use crate::session::SessionEvent;

use super::super::idle_fault::fault_from;
use super::super::idle_timeout::drain;
use super::super::outcome::DrainOutcome;

pub(super) async fn run(
    provider: &Arc<dyn Provider>,
    request: &CompletionRequest,
    session_id: &str,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
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
