use crate::tool::ToolRegistry;
use std::path::PathBuf;

/// Register workspace-scoped editing tools.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_tool_registry::register_edit_tools;
/// use codetether_agent::tool::ToolRegistry;
/// use std::path::PathBuf;
///
/// let mut registry = ToolRegistry::new();
/// register_edit_tools(&mut registry, PathBuf::from("."));
///
/// assert!(registry.contains("write"));
/// assert!(registry.contains("apply_patch"));
/// assert!(registry.contains("confirm_edit"));
/// ```
pub fn register_edit_tools(registry: &mut ToolRegistry, root_path: PathBuf) {
    codetether_worker_tools::register_edit_tools::<super::super::host::Native>(registry, root_path)
}
