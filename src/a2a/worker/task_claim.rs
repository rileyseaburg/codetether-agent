//! Task claim helpers.

use anyhow::Result;

use crate::{a2a::claim::TaskClaimResponse, provenance::ClaimProvenance};

use super::{WorkerTaskRuntime, task_timeline};

pub(super) struct ClaimedTaskData {
    pub(super) claim_provenance: ClaimProvenance,
    pub(super) provider_keys: Option<serde_json::Value>,
}

pub(super) async fn claim_task(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<Option<ClaimedTaskData>> {
    timeline.checkpoint(task_timeline::TaskCheckpoint::ClaimRequested);
    let request = super::worker_security::build_claim_request(runtime, task_id)?;
    let response = request
        .json(&serde_json::json!({ "task_id": task_id }))
        .send()
        .await?;
    if !response.status().is_success() {
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
            tracing::info!(task_id, %status, body = %body, "Task claim rejected");
        } else {
            tracing::warn!(task_id, %status, body = %body, "Failed to claim task");
        }
        return Ok(None);
    }
    let claim = response.json::<TaskClaimResponse>().await?;
    if let Some(timeout) = claim.task_timeout_seconds.map(|v| v.clamp(60, 604800)) {
        timeline.update_timeout_secs(timeout);
    }
    let provider_keys = claim.provider_keys.clone();
    let claim_provenance = claim.into_provenance();
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::Claimed,
        Some(format!(
            "run_id={:?} attempt_id={:?}",
            claim_provenance.run_id, claim_provenance.attempt_id
        )),
    );
    tracing::info!(task_id, run_id = ?claim_provenance.run_id, attempt_id = ?claim_provenance.attempt_id, "Claimed task");
    Ok(Some(ClaimedTaskData {
        claim_provenance,
        provider_keys,
    }))
}

fn summarize_response_body(body: &str) -> String {
    const MAX_BODY_CHARS: usize = 512;
    let mut summary = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if summary.chars().count() > MAX_BODY_CHARS {
        summary = summary.chars().take(MAX_BODY_CHARS).collect::<String>();
        summary.push_str("...");
    }
    summary
}
