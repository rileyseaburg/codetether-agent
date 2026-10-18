//! Execute read-only tool jobs concurrently.

use futures::future::join_all;
use tokio::sync::mpsc;

use codetether_session_events::SessionEvent;
use codetether_stream_bounds as event_payload;
use codetether_tool_execution::Host as ExecutionHost;

use super::job::Job;
use super::{result::Output, single::run_one};

pub(super) async fn execute<S: Send + 'static>(
    jobs: Vec<Job>,
    registry: &impl ExecutionHost,
    session_id: &str,
    event_tx: &mpsc::Sender<SessionEvent<S>>,
) -> Vec<Output> {
    for job in &jobs {
        let args = serde_json::to_string(&job.tool_input).unwrap_or_default();
        let _ = event_tx
            .send(SessionEvent::ToolCallStart {
                tool_call_id: job.tool_id.clone(),
                name: job.tool_name.clone(),
                arguments: event_payload::bounded_tool_arguments(&args),
            })
            .await;
    }
    join_all(
        jobs.into_iter()
            .map(|job| run_one(job, registry, session_id, event_tx)),
    )
    .await
}
