//! Deliver one output chunk after releasing the registry mutex.
use crate::Chunk;
use crate::registry::sinks;

/// Deliver output to the registered invocation, ignoring unregistered IDs.
///
/// # Arguments
/// * `id` - Registered invocation identifier.
/// * `stream` - Output stream name.
/// * `chunk` - Text to deliver without further processing.
/// # Returns
/// Nothing; the sink callback owns transport and backpressure behavior.
/// # Panics
/// Panics if the registry mutex was poisoned or the callback panics.
/// # Examples
/// ```rust
/// let _guard = codetether_tool_progress::register("call-3", "bash", |_| {});
/// codetether_tool_progress::emit("call-3", "stderr", "warning".into());
/// ```
pub fn emit(id: &str, stream: &str, chunk: String) {
    let Some(sink) = sinks().lock().unwrap().get(id).cloned() else {
        return;
    };
    (sink.send)(Chunk {
        tool_call_id: id.to_string(),
        name: sink.name,
        stream: stream.to_string(),
        chunk,
    });
}
