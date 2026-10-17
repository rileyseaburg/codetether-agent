//! Process-local bearer capability advertised only by LAN discovery.

use std::sync::OnceLock;

static TOKEN: OnceLock<String> = OnceLock::new();

/// Returns the stable, unguessable collaboration token for this process.
///
/// # Arguments
/// Takes no arguments; initializes the token lazily on first access.
/// # Returns
/// The same process-local UUID capability on every call.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::collaboration_token::local;
/// assert_eq!(local(), local());
/// assert_eq!(local().len(), 32);
/// ```
pub fn local() -> &'static str {
    TOKEN
        .get_or_init(|| uuid::Uuid::new_v4().simple().to_string())
        .as_str()
}

#[cfg(test)]
mod tests {
    #[test]
    fn token_is_stable_and_not_empty() {
        let first = super::local();
        assert_eq!(first, super::local());
        assert_eq!(first.len(), 32);
    }
}
