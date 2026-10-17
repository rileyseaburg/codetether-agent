//! Relay context context result prefix.
use super::*;

/// Build the result-key prefix used to discover relay context updates.
///
/// # Arguments
/// * `relay_id` - Relay identity.
///
/// # Returns
/// The shared prefix of context keys for this relay.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::context_result_prefix;
/// assert_eq!(context_result_prefix("demo"), "relay.demo.context.");
/// ```
pub fn context_result_prefix(relay_id: &str) -> String {
    format!("{CONTEXT_KEY_PREFIX}{relay_id}.context.")
}
