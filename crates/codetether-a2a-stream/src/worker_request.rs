//! Resumable worker task stream request construction.
use crate::{event_id::EventId, resume_request::apply_resume};
/// Build an authenticated SSE request and apply its last event cursor.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Server base URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker identifier.
/// * `name` - Agent name to advertise.
/// * `codebases` - Local codebase and workspace paths.
/// * `last` - Optional resume cursor.
/// # Returns
/// A request with SSE, worker identity, path, authentication and resume headers.
/// # Examples
/// ```rust,no_run
/// use codetether_a2a_stream::worker_request::build_stream_request;
/// let request = build_stream_request(&reqwest::Client::new(), "http://localhost:8001", &None, "worker", "agent", &[], None);
/// ```

pub fn build_stream_request(
    client: &reqwest::Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
    last: Option<&EventId>,
) -> reqwest::RequestBuilder {
    let mut request = client
        .get(format!(
            "{}/v1/worker/tasks/stream?agent_name={}&worker_id={}",
            server,
            urlencoding::encode(name),
            urlencoding::encode(worker_id)
        ))
        .header("Accept", "text/event-stream")
        .header("Accept-Encoding", "identity")
        .header("Cache-Control", "no-cache, no-transform")
        .header("X-Worker-ID", worker_id)
        .header("X-Agent-Name", name)
        .header("X-Codebases", codebases.join(","))
        .header("X-Workspaces", codebases.join(","));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    apply_resume(request, last)
}
