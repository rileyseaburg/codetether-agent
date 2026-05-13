//! Execute one read-only tool job.

use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::tool::ToolRegistry;

use super::{job::Job, result::Output};

pub(super) async fn run_one(
    job: Job,
    registry: &ToolRegistry,
    session_id: &str,
    event_tx: &mpsc::Sender<SessionEvent>,
) -> Output {
    let exec_start = std::time::Instant::now();
    let (content, success, metadata) = super::super::prompt_events::execute_tool(
        registry,
        &job.tool_name,
        &job.exec_input,
        session_id,
        exec_start,
    )
    .await;
    let duration_ms = exec_start.elapsed().as_millis() as u64;
    let _ = event_tx
        .send(SessionEvent::ToolCallComplete {
            name: job.tool_name.clone(),
            output: super::super::event_payload::bounded_tool_output(&content),
            success,
            duration_ms,
        })
        .await;
    let _ = metadata;
    Output {
        tool_id: job.tool_id,
        tool_name: job.tool_name,
        tool_input: job.tool_input,
        content,
        success,
    }
}
