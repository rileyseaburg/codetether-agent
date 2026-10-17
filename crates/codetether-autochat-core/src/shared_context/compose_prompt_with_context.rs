//! Relay context compose prompt with context.
use super::*;

/// Append a rendered context snapshot when shared memory is nonempty.
///
/// # Arguments
/// * `handoff` - Original handoff prompt.
/// * `shared` - Accumulated context buckets.
///
/// # Returns
/// The original handoff or a prompt containing the shared snapshot.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::{SharedRelayContext, compose_prompt_with_context};
/// assert_eq!(compose_prompt_with_context("review", &SharedRelayContext::default()), "review");
/// ```
pub fn compose_prompt_with_context(handoff: &str, shared: &SharedRelayContext) -> String {
    let snapshot = shared.render_prompt_snapshot();
    if snapshot.is_empty() {
        return handoff.to_string();
    }

    format!(
        "{handoff}\n\nShared Relay Context (cross-agent memory):\n{snapshot}\n\n\
Use this shared context as authoritative state across the relay."
    )
}
