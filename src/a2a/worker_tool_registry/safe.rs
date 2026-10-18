use crate::tool::ToolRegistry;
use std::path::{Path, PathBuf};

/// Register the worker's read-only tools against the current workspace root.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_tool_registry::register_safe_tools;
/// use codetether_agent::tool::ToolRegistry;
/// use std::path::PathBuf;
///
/// let mut registry = ToolRegistry::new();
/// let root = PathBuf::from(".");
/// register_safe_tools(&mut registry, root.as_path(), &root);
///
/// assert!(registry.contains("read"));
/// assert!(registry.contains("todoread"));
/// ```
pub fn register_safe_tools(registry: &mut ToolRegistry, workspace_dir: &Path, root_path: &PathBuf) {
    codetether_worker_tools::register_safe_tools::<super::host::Native>(
        registry,
        workspace_dir,
        root_path,
    )
}
