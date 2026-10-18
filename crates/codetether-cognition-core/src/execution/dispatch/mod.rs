//! Serial cognition tool dispatch through existing executable tool contracts.
//!
//! [execute_tool_requests] extracts requests with a thinker, then enforces the
//! allowlist, registry lookup, schema and lease checks in order. [run_tools]
//! obtains persona allowances and marks progress for returned execution events.
mod extract;
mod phase;
mod run;
use codetether_tool_core::Tool;
pub use extract::execute_tool_requests;
pub use phase::run_tools;
use std::sync::Arc;
/// Registry lookup required by shared cognition execution.
/// # Implementors
/// Native registries supply their existing tool handles without adapting tools.
pub trait Registry: Send + Sync {
    /// Retrieve the tool registered under the requested identifier.
    fn get(&self, id: &str) -> Option<Arc<dyn Tool>>;
}
