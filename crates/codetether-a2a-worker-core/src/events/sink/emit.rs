//! Worker event sink emit.
use super::*;

/// Emits one typed event with its human-readable text.
///
/// # Examples
///
/// # Arguments
/// * `text` - Legacy human-readable output.
/// * `event` - Typed event payload.
/// # Returns
/// Returns unit after emitting to the active sink, if any.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::events::sink::emit;
/// emit("ready".into(), serde_json::json!({"type": "ready"}));
/// ```
pub fn emit(text: String, event: serde_json::Value) {
    if let Some(sink) = sink() {
        sink(text, Some(event));
    }
}
