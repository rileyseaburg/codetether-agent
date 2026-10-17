//! Identity proof derived from the configured Forgejo principal.

use anyhow::Result;
use reqwest::RequestBuilder;

/// Attach proof for the configured runtime principal, when one exists.
///
/// # Arguments
/// * `request` - Request receiving proof headers.
/// * `action` - Authorized protocol action.
/// * `worker_id` - Worker identifier.
/// * `resource` - Canonical resource to bind.
/// # Returns
/// The request, unchanged if no runtime principal is configured.
/// # Errors
/// Returns any required canonical-worker proof configuration or signing error.
/// # Examples
/// ```rust,no_run
/// # fn example() -> anyhow::Result<()> {
/// use codetether_a2a_http::worker::proof::apply_configured;
/// let request = reqwest::Client::new().post("http://localhost:8001/task");
/// let request = apply_configured(request, "output", "worker", "resource")?;
/// # Ok(()) }
/// ```
pub fn apply_configured(
    request: RequestBuilder,
    action: &str,
    worker_id: &str,
    resource: &str,
) -> Result<RequestBuilder> {
    let Some(agent_name) = codetether_provenance::runtime_agent_identity() else {
        return Ok(request);
    };
    super::identity::apply(request, action, worker_id, &agent_name, resource)
}
