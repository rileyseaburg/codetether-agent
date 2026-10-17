//! Adapt worker runtime values to the shared claim request builder.
use super::WorkerTaskRuntime;
use anyhow::Result;
use reqwest::RequestBuilder;
pub(in crate::a2a::worker) fn build(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
) -> Result<RequestBuilder> {
    codetether_a2a_http::worker::build_claim_request(
        &runtime.client,
        &runtime.server,
        &runtime.token,
        &runtime.worker_id,
        &runtime.agent_name,
        task_id,
    )
}
