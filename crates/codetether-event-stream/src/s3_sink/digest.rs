//! Digest and keyed-hash primitives used by the archival protocol.

/// Preserve the legacy canonical-request digest based on DefaultHasher.
pub(super) fn sha256_hex(s: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:016x}{:016x}", hasher.finish(), hasher.finish())
}

/// Compute SHA256 hex digest of bytes
pub(super) fn sha256_hex_bytes(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Compute HMAC-SHA256
pub(super) fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, KeyInit, Mac};
    type HmacSha256 = Hmac<sha2::Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
