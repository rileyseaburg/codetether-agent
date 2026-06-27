//! SSE task stream HTTP request builder.

use crate::a2a::stream::event_id::EventId;
use crate::a2a::stream::resume_request::apply_resume;

use super::WorkerTaskRuntime;

pub(super) fn build_stream_request(
    runtime: &WorkerTaskRuntime,
    name: &str,
    codebases: &[String],
    last: Option<&EventId>,
) -> reqwest::RequestBuilder {
    let mut request = runtime
        .client
        .get(format!(
            "{}/v1/worker/tasks/stream?agent_name={}&worker_id={}",
            runtime.server,
            urlencoding::encode(name),
            urlencoding::encode(&runtime.worker_id)
        ))
        .header("Accept", "text/event-stream")
        .header("Accept-Encoding", "identity")
        .header("Cache-Control", "no-cache, no-transform")
        .header("X-Worker-ID", &runtime.worker_id)
        .header("X-Agent-Name", name)
        .header("X-Codebases", codebases.join(","))
        .header("X-Workspaces", codebases.join(","));
    if let Some(token) = &runtime.token {
        request = request.bearer_auth(token);
    }
    apply_resume(request, last)
}
