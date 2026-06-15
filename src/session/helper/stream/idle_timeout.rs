//! Idle-timeout guard for provider stream consumption.
//!
//! Wraps `stream.next().await` so a stalled HTTP connection (TCP open,
//! zero SSE events) cannot block the agent loop forever. When no chunk
//! arrives within [`IDLE_TIMEOUT`] the stream is treated as ended and the
//! partially accumulated response is returned instead of hanging the TUI.

use crate::provider::{StreamChunk, Usage};
use crate::session::SessionEvent;
use anyhow::Result;
use futures::StreamExt;
use futures::stream::BoxStream;
use std::collections::HashMap;
use std::time::Duration;

use super::finalize::ToolAccumulator;
use super::{empty, finalize, text_acc, tool_acc};

/// Maximum gap between consecutive stream chunks before the stream is
/// considered stalled. GLM-5.2 can think 60-90 s before the first token,
/// so 3 min gives generous headroom.
pub(super) const IDLE_TIMEOUT: Duration = Duration::from_secs(180);

/// Drain `stream`, accumulating text / thinking / tool calls with an idle
/// timeout. Returns the assembled completion; on timeout the partial
/// response is returned so the agent loop continues instead of hanging.
pub(super) async fn drain(
    mut stream: BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> Result<crate::provider::CompletionResponse> {
    let mut text = String::new();
    let mut thinking = String::new();
    let mut tools = Vec::<ToolAccumulator>::new();
    let mut idx: HashMap<String, usize> = HashMap::new();
    let mut usage = Usage::default();

    loop {
        let chunk = match tokio::time::timeout(IDLE_TIMEOUT, stream.next()).await {
            Ok(Some(c)) => c,
            Ok(None) => break,
            Err(_) => {
                tracing::warn!(timeout_secs = IDLE_TIMEOUT.as_secs(), "Stream idle timeout — returning partial");
                break;
            }
        };
        match chunk {
            StreamChunk::Text(d) => text_acc::on_text(&mut text, &d, event_tx).await?,
            StreamChunk::ToolCallStart { id, name } => tool_acc::on_tool_start(&mut tools, &mut idx, id, name),
            StreamChunk::ToolCallDelta { id, arguments_delta } => {
                tool_acc::on_tool_delta(&mut tools, &mut idx, id, arguments_delta)?;
            }
            StreamChunk::ToolCallEnd { .. } => {}
            StreamChunk::Thinking(d) => text_acc::on_thinking(&mut thinking, &d, event_tx)?,
            StreamChunk::Done { usage: u } => { if let Some(u) = u { usage = u; } }
            StreamChunk::Error(msg) => anyhow::bail!(msg),
        }
    }
    empty::reject(finalize::build_response(thinking, text, tools, usage))
}
