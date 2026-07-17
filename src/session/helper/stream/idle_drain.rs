//! Per-chunk dispatch for the idle-timeout stream drain.
//!
//! Splitting chunk handling out of [`super::idle_timeout::drain`] keeps that
//! module within the line budget and isolates the per-variant accumulation
//! logic in one focused place.

use crate::provider::StreamChunk;
use crate::session::SessionEvent;
use anyhow::Result;

use super::drain_state::DrainState;
use super::{reasoning, text_acc, tool_acc};

/// Apply a single [`StreamChunk`] to the accumulating [`DrainState`].
pub(super) async fn apply(
    state: &mut DrainState,
    chunk: StreamChunk,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    match chunk {
        StreamChunk::Text(d) => text_acc::on_text(&mut state.text, &d, event_tx).await?,
        StreamChunk::ToolCallStart { id, name } => {
            tool_acc::on_tool_start(&mut state.tools, &mut state.idx, id, name)
        }
        StreamChunk::ToolCallDelta {
            id,
            arguments_delta,
        } => tool_acc::on_tool_delta(&mut state.tools, &mut state.idx, id, arguments_delta)?,
        StreamChunk::ToolCallEnd { .. } => {}
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
