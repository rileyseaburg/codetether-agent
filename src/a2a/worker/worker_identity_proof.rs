//! Possession proof for canonical Forgejo worker identities.

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use reqwest::RequestBuilder;

use codetether_provenance::worker_proof::sign;

pub(in crate::a2a::worker) fn apply(
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
