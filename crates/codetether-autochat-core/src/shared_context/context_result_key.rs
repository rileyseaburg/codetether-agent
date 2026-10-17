//! Relay context context result key.
use super::*;

/// Build a stable result key for a numbered context delta.
///
/// # Arguments
/// * `relay_id` - Relay identity.
/// * `turn` - Turn index used in the key.
///
/// # Returns
/// A relay-scoped, zero-padded result key.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::context_result_key;
/// assert_eq!(context_result_key("demo", 2), "relay.demo.context.0002");
/// ```
pub fn context_result_key(relay_id: &str, turn: usize) -> String {
    format!("{CONTEXT_KEY_PREFIX}{relay_id}.context.{turn:04}")
}
