//! Complete-output hashes and bounded UTF-8-safe excerpts.
use sha2::{Digest, Sha256};

const PREVIEW_BYTES: usize = 512;

pub(super) fn sha256_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hex::encode(hasher.finalize())
}

pub(super) fn preview(value: &str) -> String {
    codetether_util::truncate::truncate_bytes_safe(value, PREVIEW_BYTES).to_string()
}
