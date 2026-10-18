//! Task classification and advertised-tool filtering.
use crate::SubTask;
use codetether_provider_core::ToolDefinition;
use codetether_tool_core::readonly::is_read_only;

/// Return the task's established read-only classification.
/// # Arguments
/// Receives the shared task metadata.
/// # Returns
/// True when the existing task policy forbids mutation.
/// # Examples
/// ```
/// let task = codetether_swarm_core::SubTask::new("Research", "Find unsafe usages")
///     .with_specialty("Researcher").with_needs_worktree(false);
/// assert!(codetether_swarm_core::tool_policy::is_read_only_task(&task));
/// ```
pub fn is_read_only_task(task: &SubTask) -> bool {
    task.is_read_only()
}

/// Filter advertised tool definitions for autonomous execution.
/// # Arguments
/// Receives all definitions and the read-only flag.
/// # Returns
/// A cloned list without interactive or disallowed mutating tools.
/// # Examples
/// ```
/// assert!(codetether_swarm_core::tool_policy::definitions(&[], true).is_empty());
/// ```
pub fn definitions(all: &[ToolDefinition], read_only: bool) -> Vec<ToolDefinition> {
    all.iter()
        .filter(|tool| tool.name != "question")
        .filter(|tool| !read_only || is_read_only(&tool.name))
        .cloned()
        .collect()
}
