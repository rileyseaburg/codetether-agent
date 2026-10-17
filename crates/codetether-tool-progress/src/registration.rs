//! Register live output callbacks and remove them when their scope ends.
use crate::Chunk;
use crate::registry::{Sink, sinks};
use std::sync::Arc;

/// Registration lifetime; dropping it removes the sink for its invocation ID.
///
/// # Examples
/// ```rust
/// let guard = codetether_tool_progress::register("call-1", "bash", |_| {});
/// drop(guard);
/// ```
pub struct Guard {
    id: String,
}

/// Register an invocation's output callback until the returned guard is dropped.
///
/// # Arguments
/// * `id` - Invocation identifier; an existing registration is replaced.
/// * `name` - Tool name included in delivered chunks.
/// * `send` - Callback invoked outside the lock; use nonblocking delivery.
/// # Returns
/// A guard that removes the ID on drop, including any replacement at that ID.
/// # Panics
/// Panics if the registry mutex was poisoned.
/// # Examples
/// ```rust
/// let _guard = codetether_tool_progress::register("call-2", "bash", |chunk| {
///     assert_eq!(chunk.name, "bash");
/// });
/// codetether_tool_progress::emit("call-2", "stdout", "hello".into());
/// ```
pub fn register(id: &str, name: &str, send: impl Fn(Chunk) + Send + Sync + 'static) -> Guard {
    sinks().lock().unwrap().insert(
        id.to_string(),
        Sink {
            name: name.to_string(),
            send: Arc::new(send),
        },
    );
    Guard { id: id.to_string() }
}

impl Drop for Guard {
    fn drop(&mut self) {
        sinks().lock().unwrap().remove(&self.id);
    }
}
