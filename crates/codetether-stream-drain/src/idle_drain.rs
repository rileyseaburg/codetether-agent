//! Per-chunk dispatch for the idle-timeout stream drain.
//!
//! Splitting chunk handling out of [`super::idle_timeout::drain`] keeps that
//! module within the line budget and isolates the per-variant accumulation
//! logic in one focused place.

use anyhow::Result;
use codetether_provider_core::StreamChunk;
use codetether_session_events::SessionEvent;

use super::drain_state::DrainState;
use super::{reasoning, text_acc, tool_delta, tool_start};

/// Apply a single [`StreamChunk`] to the accumulating [`DrainState`].
pub(super) async fn apply<S>(
    state: &mut DrainState,
    chunk: StreamChunk,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
) -> Result<()> {
    match chunk {
        StreamChunk::KeepAlive => {}
        StreamChunk::Text(d) => text_acc::on_text(&mut state.text, &d, event_tx).await?,
        StreamChunk::ToolCallStart { id, name } => {
            tool_start::on_tool_start(&mut state.tools, &mut state.idx, id, name)
        }
        StreamChunk::ToolCallDelta {
            id,
            arguments_delta,
        } => tool_delta::on_tool_delta(&mut state.tools, &mut state.idx, id, arguments_delta)?,
        StreamChunk::ToolCallEnd { .. } => {}
        StreamChunk::OutputItemDone { content } => state.completed.push(content),
        StreamChunk::Thinking(delta) => reasoning::apply(state, delta, event_tx)?,
        StreamChunk::Done { usage: u } => {
            if let Some(u) = u {
                state.usage = u;
            }
        }
        // Terminal errors are intercepted by the drain loop and classified as
        // StreamStop::Fault before reaching here; treat as no-op defensively.
        StreamChunk::Error(_) => {}
    }
    Ok(())
}
