//! Build a fresh authenticated task-stream request for each connection attempt.
use crate::identity::Identity;
use reqwest::{Client, RequestBuilder};
pub(crate) fn build(id: &Identity) -> RequestBuilder {
    let url = format!(
        "{}/v1/worker/tasks/stream?agent_name={}&worker_id={}",
        id.server,
        urlencoding::encode(&id.worker_name),
        urlencoding::encode(&id.worker_id)
    );
    Client::new()
        .get(&url)
        .header("Accept", "text/event-stream")
        .header("X-Worker-ID", &id.worker_id)
        .header("X-Agent-Name", &id.worker_name)
        .bearer_auth(&id.token)
}
