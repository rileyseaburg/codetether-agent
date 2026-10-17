//! Structured worker event is sensitive key.
use super::*;

/// Returns true when an argument name looks like it carries a secret.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::events::is_sensitive_key;
/// assert!(is_sensitive_key("GITHUB_TOKEN"));
/// assert!(!is_sensitive_key("path"));
/// ```
/// # Arguments
/// * `key` - Argument field name to inspect.
/// # Returns
/// Whether the name contains a configured sensitive-key fragment.
pub fn is_sensitive_key(key: &str) -> bool {
    let lowered = key.to_ascii_lowercase();
    SENSITIVE_KEYS.iter().any(|needle| lowered.contains(needle))
}
