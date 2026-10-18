use crate::tool::ToolRegistry;

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
    codetether_worker_tools::register_task_tools::<super::super::host::Native>(registry)
}
