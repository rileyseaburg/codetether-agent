//! HMAC and digest primitives shared with presigned token generation.
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
/// HMAC-SHA256 helper returning raw bytes.
///
/// # Arguments
/// * `key` - HMAC key bytes.
/// * `data` - Payload to authenticate.
///
/// # Returns
/// The 32-byte authentication code.
///
/// # Panics
/// Panics if the HMAC implementation rejects the key length.
///
/// # Examples
/// ```
/// assert_eq!(codetether_bedrock_auth::sigv4::hmac_sha256(b"key", b"data").len(), 32);
/// ```
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
/// SHA-256 helper returning lowercase hex.
///
/// # Arguments
/// * `data` - Bytes to hash.
///
/// # Returns
/// The lowercase, 64-character SHA-256 digest.
///
/// # Examples
/// ```
/// assert_eq!(codetether_bedrock_auth::sigv4::sha256_hex(b"").len(), 64);
/// ```
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
