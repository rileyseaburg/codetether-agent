//! Hashing helpers for the local embedding engine.

use sha2::{Digest, Sha256};

/// Accumulate hashed character trigrams when no word tokens are present.
pub(super) fn accumulate_char_ngrams(vector: &mut [f32], input: &str) {
    for ngram in input.as_bytes().windows(3).take(2048) {
        let key = String::from_utf8_lossy(ngram);
        accumulate_token(vector, &key, 0.5);
    }
}

/// Hash `token` into three signed buckets and add weighted contributions.
pub(super) fn accumulate_token(vector: &mut [f32], token: &str, weight: f32) {
    if token.is_empty() {
        return;
    }
    let digest = Sha256::digest(token.as_bytes());
    let len = vector.len();
    let idx_a = (u16::from_le_bytes([digest[0], digest[1]]) as usize) % len;
    let idx_b = (u16::from_le_bytes([digest[2], digest[3]]) as usize) % len;
    let idx_c = (u16::from_le_bytes([digest[4], digest[5]]) as usize) % len;
    let sign = |b: u8| if b & 1 == 0 { 1.0 } else { -1.0 };
    vector[idx_a] += sign(digest[6]) * weight;
    vector[idx_b] += sign(digest[7]) * (weight * 0.7);
    vector[idx_c] += sign(digest[8]) * (weight * 0.4);
}
