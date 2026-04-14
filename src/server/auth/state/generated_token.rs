use rand::RngExt;

/// Generate a random fallback bearer token.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::server::auth::generate_token;
///
/// let token = generate_token();
/// assert_eq!(token.len(), 64);
/// assert!(token.chars().all(|ch| ch.is_ascii_hexdigit()));
/// ```
pub fn generate_token() -> String {
    let generated: String = {
        let mut rng = rand::rng();
        (0..32)
            .map(|_| format!("{:02x}", rng.random::<u8>()))
            .collect()
    };
    tracing::warn!(token = %generated, "No CODETETHER_AUTH_TOKEN set — generated a random token. Set CODETETHER_AUTH_TOKEN to use a stable token.");
    generated
}
