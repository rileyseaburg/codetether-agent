use crate::tool::{ToolRegistry, mcp_bridge, plan, prd, task};
use std::sync::Arc;

/// Register non-provider workflow tools for mutable worker tasks.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_tool_registry::register_task_tools;
/// use codetether_agent::tool::ToolRegistry;
///
/// let mut registry = ToolRegistry::new();
/// register_task_tools(&mut registry);
///
/// assert!(registry.contains("task"));
/// assert!(registry.contains("plan_enter"));
/// assert!(registry.contains("prd"));
/// ```
pub fn register_task_tools(registry: &mut ToolRegistry) {
    registry.register(Arc::new(task::TaskTool::new()));
    registry.register(Arc::new(plan::PlanEnterTool::new()));
    registry.register(Arc::new(plan::PlanExitTool::new()));
    registry.register(Arc::new(prd::PrdTool::new()));
    registry.register(Arc::new(mcp_bridge::McpBridgeTool::new()));
}
