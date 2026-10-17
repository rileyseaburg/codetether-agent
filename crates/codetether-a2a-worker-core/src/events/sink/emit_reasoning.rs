//! Worker event sink emit reasoning.
use super::*;

/// Emits an `agent.reasoning` event when the provider exposed reasoning text.
/// # Arguments
/// * `text` - Reasoning text; blank content is ignored.
/// # Returns
/// Returns unit after attempting to emit nonblank reasoning.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::events::sink::emit_reasoning;
/// emit_reasoning("Checking the result.");
/// ```
pub fn emit_reasoning(text: &str) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return;
    }
    emit(String::new(), super::super::reasoning_event(trimmed));
}
