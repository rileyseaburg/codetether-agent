//! Execute read-only tool jobs concurrently.

use futures::future::join_all;
use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::tool::ToolRegistry;

use super::job::Job;
use super::{result::Output, single::run_one};

pub(super) async fn execute(
    jobs: Vec<Job>,
    registry: &ToolRegistry,
    session_id: &str,
    event_tx: &mpsc::Sender<SessionEvent>,
) -> Vec<Output> {
    for job in &jobs {
        let args = serde_json::to_string(&job.tool_input).unwrap_or_default();
        let _ = event_tx
            .send(SessionEvent::ToolCallStart {
                name: job.tool_name.clone(),
                arguments: super::super::event_payload::bounded_tool_arguments(&args),
            })
            .await;
    }
    join_all(
        jobs.into_iter()
            .map(|job| run_one(job, registry, session_id, event_tx)),
    )
    .await
}
