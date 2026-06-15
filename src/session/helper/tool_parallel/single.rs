//! Execute one read-only tool job.

use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::session::helper::{event_payload, prompt_events, tool_metadata_event};
use crate::tool::ToolRegistry;

use super::{job::Job, result::Output};

pub(super) async fn run_one(
    job: Job,
    registry: &ToolRegistry,
    session_id: &str,
    event_tx: &mpsc::Sender<SessionEvent>,
) -> Output {
    let exec_start = std::time::Instant::now();
    // Keep the watchdog alive for slow parallel tools, mirroring the serial
    // path in `prompt_events`. Without this, a long read-only tool emits no
    // activity events and can trip a false stall detection.
    let hb = super::super::super::tool_heartbeat::spawn(
        event_tx,
        &job.tool_id,
        &job.tool_name,
        exec_start,
    );
    let (content, success, metadata) = prompt_events::execute_tool(
        registry,
        &job.tool_name,
        &job.exec_input,
        session_id,
        exec_start,
        Some((event_tx, &job.tool_id)),
    )
    .await;
    hb.abort();
    let duration_ms = exec_start.elapsed().as_millis() as u64;
    tool_metadata_event::send(event_tx, &job.tool_id, &job.tool_name, metadata.as_ref()).await;
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
    }
}
