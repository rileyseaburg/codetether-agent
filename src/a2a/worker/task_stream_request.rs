//! Adapt worker runtime fields to the shared stream request builder.
use super::WorkerTaskRuntime;
use crate::a2a::stream::event_id::EventId;
pub(super) fn build_stream_request(
    runtime: &WorkerTaskRuntime,
    name: &str,
    codebases: &[String],
    last: Option<&EventId>,
) -> reqwest::RequestBuilder {
    codetether_a2a_stream::worker_request::build_stream_request(
        &runtime.client,
        &runtime.server,
        &runtime.token,
        &runtime.worker_id,
        name,
        codebases,
        last,
    )
}
