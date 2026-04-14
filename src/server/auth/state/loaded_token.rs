use super::generate_token;

/// Load the configured token or fall back to a generated one.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::server::auth::load_token;
///
/// # unsafe {
/// std::env::set_var("CODETETHER_AUTH_TOKEN", "example-token");
/// # }
/// assert_eq!(load_token(), "example-token");
/// ```
pub fn load_token() -> String {
    match std::env::var("CODETETHER_AUTH_TOKEN") {
        Ok(token) if !token.is_empty() => {
            tracing::info!(source = "CODETETHER_AUTH_TOKEN", "Auth token loaded");
            token
        }
        _ => generate_token(),
    }
}
