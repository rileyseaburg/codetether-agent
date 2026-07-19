//! Identity proof derived from the configured Forgejo principal.

use anyhow::Result;
use reqwest::RequestBuilder;

pub(in crate::a2a::worker) fn apply(
    request: RequestBuilder,
    action: &str,
    worker_id: &str,
    resource: &str,
) -> Result<RequestBuilder> {
    let Some(agent_name) = crate::provenance::runtime_agent_identity() else {
        return Ok(request);
    };
    super::worker_identity_proof::apply(request, action, worker_id, &agent_name, resource)
}
