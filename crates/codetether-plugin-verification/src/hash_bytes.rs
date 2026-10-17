//! hash bytes for verified plugin registration.
use super::*;

/// Compute SHA-256 hash of byte content.
pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
