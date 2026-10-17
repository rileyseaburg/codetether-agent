//! Canonical worker possession-proof signing.
use anyhow::Result;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

pub fn sign(
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
#[path = "sign_tests.rs"]
mod tests;
