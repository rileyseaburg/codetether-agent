//! Preserve HTTP claim rejection diagnostics and progress.
use super::*;
pub(super) async fn reject(
    task_id: &str,
    response: reqwest::Response,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<Option<ClaimedTaskData>> {
    let status = response.status();
    let body = response
        .text()
        .await
        .unwrap_or_else(|error| format!("<failed to read response body: {error}>"));
    let body = summarize_response_body(&body);
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::ClaimRejected,
        Some(format!("status={status} body={body}")),
    );
    if status == reqwest::StatusCode::CONFLICT {
        tracing::info!(target: "codetether_agent::a2a::worker::task_claim", task_id, %status, body = %body, "Task claim rejected");
    } else {
        tracing::warn!(target: "codetether_agent::a2a::worker::task_claim", task_id, %status, body = %body, "Failed to claim task");
    }
    return Ok(None);
}
