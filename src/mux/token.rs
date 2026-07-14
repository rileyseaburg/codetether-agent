//! Mux bearer-token generation and validation.

use rand::RngExt;

pub(super) const BOOTSTRAP_ENV: &str = "CODETETHER_MUX_BOOTSTRAP_TOKEN";

pub(super) fn generate() -> String {
    let mut rng = rand::rng();
    (0..32)
        .map(|_| format!("{:02x}", rng.random::<u8>()))
        .collect()
}

pub(super) fn matches(provided: &str, expected: &str) -> bool {
    let left = provided.as_bytes();
    let right = expected.as_bytes();
    let mut diff = (left.len() ^ right.len()) as u8;
    for index in 0..left.len().max(right.len()) {
        diff |= left.get(index).copied().unwrap_or(0) ^ right.get(index).copied().unwrap_or(0);
    }
    diff == 0
}
