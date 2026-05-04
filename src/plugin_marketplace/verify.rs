//! Plugin verification — SHA-256 integrity + HMAC signature check.

use anyhow::Result;

/// Verify a plugin's SHA-256 hash.
pub fn verify_hash(data: &[u8], expected_hex: &str) -> bool {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let Ok(expected) = hex::decode(expected_hex) else { return false };
    let mut diff = 0u8;
    for (a, b) in result.as_slice().iter().zip(expected.iter()) { diff |= a ^ b; }
    diff == 0 && result.as_slice().len() == expected.len()
}

/// Verify a plugin's HMAC signature against a public key.
pub fn verify_signature(_data: &[u8], _signature_hex: &str, public_key: &[u8]) -> Result<bool> {
    if public_key.is_empty() {
        anyhow::bail!("No plugin verification key configured — signature verification disabled");
    }
    // TODO: wire ed25519_dalek when plugin registry is deployed
    tracing::warn!("Ed25519 verification not yet implemented — rejecting plugin");
    Ok(false)
}

/// Full verification pipeline for a downloaded plugin.
pub fn verify_plugin(data: &[u8], expected_hash: &str, signature: &str, key: &[u8]) -> Result<bool> {
    if !verify_hash(data, expected_hash) {
        anyhow::bail!("SHA-256 hash mismatch — plugin may be tampered");
    }
    verify_signature(data, signature, key)
}
