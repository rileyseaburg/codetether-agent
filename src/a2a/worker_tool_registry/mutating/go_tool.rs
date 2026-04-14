use crate::tool::{ToolRegistry, go};
use std::sync::Arc;

/// Register the background `go` tool, optionally wiring a completion callback.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_tool_registry::register_go_tool;
/// use codetether_agent::tool::ToolRegistry;
///
/// let mut registry = ToolRegistry::new();
/// register_go_tool(&mut registry, None);
///
/// assert!(registry.contains("go"));
/// ```
pub fn register_go_tool(
    registry: &mut ToolRegistry,
    completion_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    registry.register(Arc::new(match completion_callback {
        Some(callback) => go::GoTool::with_callback(callback),
        None => go::GoTool::new(),
    }));
}
