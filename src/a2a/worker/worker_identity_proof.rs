//! Possession proof for canonical Forgejo worker identities.

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use hmac::{Hmac, Mac};
use reqwest::RequestBuilder;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

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

fn sign(
    secret: &str,
    action: &str,
    worker_id: &str,
    name: &str,
    resource: &str,
    timestamp: &str,
) -> Result<String> {
    let payload = format!(
        "codetether-worker-proof-v1\n{action}\n{worker_id}\n{name}\n{resource}\n{timestamp}"
    );
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
    mac.update(payload.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

#[cfg(test)]
#[path = "worker_identity_proof_tests.rs"]
mod tests;
