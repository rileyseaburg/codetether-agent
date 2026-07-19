//! Authenticated request construction for claiming one worker task.

use anyhow::Result;
use reqwest::RequestBuilder;

use super::WorkerTaskRuntime;

pub(in crate::a2a::worker) fn build(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
) -> Result<RequestBuilder> {
    let request = runtime
        .client
        .post(format!("{}/v1/worker/tasks/claim", runtime.server))
        .header("X-Worker-ID", &runtime.worker_id);
    let request = super::worker_identity_proof::apply(
        request,
        "claim",
        &runtime.worker_id,
        &runtime.agent_name,
        task_id,
    )?;
    Ok(match &runtime.token {
        Some(token) => request.bearer_auth(token),
        None => request,
    })
}
