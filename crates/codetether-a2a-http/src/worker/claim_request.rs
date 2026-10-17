//! Authenticated worker task-claim request construction.
use anyhow::Result;
use reqwest::RequestBuilder;
/// Build a claim request with worker identity proof and optional bearer authentication.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker identifier.
/// * `agent_name` - Principal used for canonical proof.
/// * `task_id` - Task bound by that proof.
/// # Returns
/// A request ready for its task JSON body.
/// # Errors
/// Returns required proof configuration or signing errors.
/// # Examples
/// ```rust,no_run
/// # fn example() -> anyhow::Result<()> {
/// use codetether_a2a_http::worker::build_claim_request;
/// let request = build_claim_request(&reqwest::Client::new(), "http://localhost:8001", &None, "worker", "agent", "task")?;
/// # Ok(()) }
/// ```

pub fn build_claim_request(
    client: &reqwest::Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    agent_name: &str,
    task_id: &str,
) -> Result<RequestBuilder> {
    let request = client
        .post(format!("{}/v1/worker/tasks/claim", server))
        .header("X-Worker-ID", worker_id);
    let request = super::proof::apply(request, "claim", worker_id, agent_name, task_id)?;
    Ok(match token {
        Some(token) => request.bearer_auth(token),
        None => request,
    })
}
