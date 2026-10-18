//! Mux bearer-token generation and validation.
//!
//! [generate] creates a bootstrap credential; [BOOTSTRAP_ENV] names its child
//! process environment entry. [matches] compares presented and stored tokens.

use rand::RngExt;

/// Environment entry through which the CLI supplies a server bootstrap token.
pub const BOOTSTRAP_ENV: &str = "CODETETHER_MUX_BOOTSTRAP_TOKEN";

/// Generate a 32-byte random token encoded as hexadecimal.
///
/// # Returns
/// A 64-character token for authenticating mux clients.
/// # Examples
/// ```
/// assert_eq!(codetether_mux_server::token::generate().len(), 64);
/// ```
pub fn generate() -> String {
    let mut rng = rand::rng();
    (0..32)
        .map(|_| format!("{:02x}", rng.random::<u8>()))
        .collect()
}

/// Compare supplied and stored token bytes.
///
/// # Arguments
/// * `provided` - Token presented by the client.
/// * `expected` - Stored server token.
/// # Returns
/// Whether the byte comparison accepts the supplied token.
/// # Examples
/// ```
/// assert!(codetether_mux_server::token::matches("abc", "abc"));
/// assert!(!codetether_mux_server::token::matches("abc", "def"));
/// ```
pub fn matches(provided: &str, expected: &str) -> bool {
    let left = provided.as_bytes();
    let right = expected.as_bytes();
    let mut diff = (left.len() ^ right.len()) as u8;
    for index in 0..left.len().max(right.len()) {
        diff |= left.get(index).copied().unwrap_or(0) ^ right.get(index).copied().unwrap_or(0);
    }
    diff == 0
}
