//! Possession proof for canonical Forgejo worker identities.

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use reqwest::RequestBuilder;

use codetether_provenance::worker_proof::sign;

/// Attach a canonical Forgejo worker possession proof when required by its name.
///
/// # Arguments
/// * `request` - Request receiving proof headers.
/// * `action` - Protocol action being authorized.
/// * `worker_id` - Worker identifier bound by the signature.
/// * `name` - Worker principal; noncanonical names leave the request unchanged.
/// * `resource` - Canonical resource bound by the signature.
/// # Returns
/// The request with key, timestamp, and proof headers when applicable.
/// # Errors
/// Returns missing credential, clock, or signing errors for canonical workers.
/// # Examples
/// ```rust,no_run
/// # fn example() -> anyhow::Result<()> {
/// use codetether_a2a_http::worker::proof::apply;
/// let request = reqwest::Client::new().post("http://localhost:8001/task");
/// let request = apply(request, "claim", "worker", "ctforgejo_example", "task-1")?;
/// # Ok(()) }
/// ```
pub fn apply(
    request: RequestBuilder,
    action: &str,
    worker_id: &str,
    name: &str,
    resource: &str,
) -> Result<RequestBuilder> {
    if !name.starts_with("ctforgejo_") {
        return Ok(request);
    }
    let key_id = std::env::var("CODETETHER_KEY_ID")
        .context("CODETETHER_KEY_ID is required for canonical Forgejo workers")?;
    let secret = std::env::var("CODETETHER_SIGNING_SECRET")
        .context("CODETETHER_SIGNING_SECRET is required for canonical Forgejo workers")?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .to_string();
    let signature = sign(&secret, action, worker_id, name, resource, &timestamp)?;
    Ok(request
        .header("X-CodeTether-Key-ID", key_id)
        .header("X-CodeTether-Proof-Timestamp", timestamp)
        .header("X-CodeTether-Worker-Proof", signature))
}
