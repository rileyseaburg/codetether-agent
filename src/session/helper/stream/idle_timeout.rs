//! Idle-timeout guard + stop classification for provider stream consumption.
//!
//! Wraps `stream.next().await` so a stalled HTTP connection (TCP open,
//! zero SSE events) cannot block the agent loop forever, and classifies *why*
//! the stream stopped into a [`StreamStop`] so the restart engine can decide
//! whether re-opening a fresh stream is sound.

use crate::provider::StreamChunk;
use crate::session::SessionEvent;
use futures::StreamExt;
use futures::stream::BoxStream;
use std::time::Duration;

use super::idle_drain::{DrainState, apply};
use super::outcome::{DrainOutcome, StreamStop};
use super::{fault, finalize};

/// Maximum gap between consecutive stream chunks before the stream is
/// considered stalled. GLM-5.2 can think 60-90 s before the first token,
/// so 3 min gives generous headroom.
pub(super) const IDLE_TIMEOUT: Duration = Duration::from_secs(180);

fn fault_from(msg: &str) -> StreamStop {
    StreamStop::Fault {
        transient: fault::is_transient(msg),
    }
}

/// Drain `stream`, classifying the stop reason. Never errors on its own; a
/// terminal error chunk is reported as [`StreamStop::Fault`] for the caller.
pub(super) async fn drain(
    mut stream: BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> DrainOutcome {
    let mut state = DrainState::new();
    let mut got_chunk = false;
    let stop = loop {
        match tokio::time::timeout(IDLE_TIMEOUT, stream.next()).await {
            Ok(Some(StreamChunk::Error(msg))) => break fault_from(&msg),
            Ok(Some(c)) => {
                got_chunk = true;
                if let Err(e) = apply(&mut state, c, event_tx).await {
                    break fault_from(&e.to_string());
                }
            }
            Ok(None) => break StreamStop::Clean,
            Err(_) if !got_chunk => {
                tracing::warn!("Cold idle timeout; retryable");
                break StreamStop::ColdStall;
            }
            Err(_) => {
                tracing::warn!("Idle timeout; returning partial");
                break StreamStop::MidStreamStall;
            }
        }
    };
    let response = finalize::build_response(state.thinking, state.text, state.tools, state.usage);
    DrainOutcome {
        response: Some(response),
        stop,
    }
}
