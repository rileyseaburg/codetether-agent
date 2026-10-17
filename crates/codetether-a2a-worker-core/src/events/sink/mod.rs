//! Active worker event callback and emission helpers.
//!
//! [install_sink] replaces the process-wide callback. Emitters do nothing when
//! no callback is installed, preserving behavior for non-worker callers.
use std::sync::{Arc, RwLock};
/// Callback receiving legacy output and an optional structured event.
///
/// # Examples
/// ```rust
/// use std::sync::Arc;
/// use codetether_a2a_worker_core::events::sink::EventSink;
/// let sink: EventSink = Arc::new(|_, _| {});
/// sink("ready".into(), None);
/// ```
pub type EventSink = Arc<dyn Fn(String, Option<serde_json::Value>) + Send + Sync + 'static>;
static ACTIVE_SINK: RwLock<Option<EventSink>> = RwLock::new(None);
mod current;
use current::sink;
mod install_sink;
pub use install_sink::install_sink;
mod emit;
pub use emit::emit;
mod emit_tool_call_if_active;
pub use emit_tool_call_if_active::emit_tool_call_if_active;
mod emit_tool_result_if_active;
pub use emit_tool_result_if_active::emit_tool_result_if_active;
mod emit_tool_call;
pub use emit_tool_call::emit_tool_call;
mod emit_tool_result;
pub use emit_tool_result::emit_tool_result;
mod emit_reasoning;
pub use emit_reasoning::emit_reasoning;
#[cfg(test)]
#[path = "session_event_sink_tests.rs"]
mod tests;
