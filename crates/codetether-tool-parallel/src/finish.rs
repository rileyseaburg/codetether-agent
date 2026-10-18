//! Emit completion metadata and retain image attachments from one tool result.
use crate::{Output, job::Job};
use codetether_session_events::SessionEvent;
use codetether_stream_bounds as event_payload;
use codetether_tool_reporting::{self as tool_metadata_event, ToolTuple};
use tokio::sync::mpsc;
pub(super) async fn finish<S>(
    job: Job,
    result: ToolTuple,
    duration_ms: u64,
    event_tx: &mpsc::Sender<SessionEvent<S>>,
) -> Output {
    let (content, success, metadata) = result;
    tool_metadata_event::send_metadata(event_tx, &job.tool_id, &job.tool_name, metadata.as_ref())
        .await;
    let _ = event_tx
        .send(SessionEvent::ToolCallComplete {
            tool_call_id: job.tool_id.clone(),
            name: job.tool_name.clone(),
            output: event_payload::bounded_tool_output(&content),
            success,
            duration_ms,
        })
        .await;
    Output {
        tool_id: job.tool_id,
        tool_name: job.tool_name,
        tool_input: job.tool_input,
        content,
        success,
        images: codetether_tool_core::images::content(metadata.as_ref()),
    }
}
