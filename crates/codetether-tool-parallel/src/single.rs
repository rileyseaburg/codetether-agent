//! Execute one read-only tool job.

use tokio::sync::mpsc;

use codetether_session_events::SessionEvent;
use codetether_tool_execution::Host as ExecutionHost;

use super::{job::Job, result::Output};

pub(super) async fn run_one<S: Send + 'static>(
    job: Job,
    registry: &impl ExecutionHost,
    session_id: &str,
    event_tx: &mpsc::Sender<SessionEvent<S>>,
) -> Output {
    let exec_start = std::time::Instant::now();
    // Keep the watchdog alive for slow parallel tools, mirroring the serial
    // path in `prompt_events`. Without this, a long read-only tool emits no
    // activity events and can trip a false stall detection.
    let hb = codetether_tool_reporting::spawn_heartbeat(
        event_tx,
        &job.tool_id,
        &job.tool_name,
        exec_start,
    );
    let (content, success, metadata) = codetether_tool_execution::execute(
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
    crate::finish::finish(job, (content, success, metadata), duration_ms, event_tx).await
}
