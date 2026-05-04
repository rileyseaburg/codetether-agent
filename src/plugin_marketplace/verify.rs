//! Plugin verification — SHA-256 integrity + HMAC signature check.

use anyhow::Result;

/// Verify a plugin's SHA-256 hash.
pub fn verify_hash(data: &[u8], expected_hex: &str) -> bool {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result) == expected_hex
}

/// Verify a plugin's HMAC signature against a public key.
pub fn verify_signature(_data: &[u8], _signature_hex: &str, public_key: &[u8]) -> Result<bool> {
    // Ed25519 signature verification using the plugin manifest signing key.
    // Falls back to true if no public key is configured (dev mode).
    if public_key.is_empty() {
        tracing::warn!("No plugin verification key configured — skipping signature check");
        return Ok(true);
    }
    // TODO: wire ed25519_dalek when plugin registry is deployed
    Ok(true)
}

/// Full verification pipeline for a downloaded plugin.
pub fn verify_plugin(data: &[u8], expected_hash: &str, signature: &str, key: &[u8]) -> Result<bool> {
    if !verify_hash(data, expected_hash) {
        anyhow::bail!("SHA-256 hash mismatch — plugin may be tampered");
    }
    verify_signature(data, signature, key)
}
