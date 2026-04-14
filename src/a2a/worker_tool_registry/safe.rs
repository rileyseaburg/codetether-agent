use crate::tool::{ToolRegistry, codesearch, file, lsp, search, skill, todo, webfetch, websearch};
use std::path::{Path, PathBuf};
use std::sync::Arc;

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
    registry.register(Arc::new(file::ReadTool::new()));
    registry.register(Arc::new(file::ListTool::new()));
    registry.register(Arc::new(file::GlobTool::with_root(root_path.clone())));
    registry.register(Arc::new(search::GrepTool::new()));
    registry.register(Arc::new(lsp::LspTool::with_root(format!(
        "file://{}",
        workspace_dir.display()
    ))));
    registry.register(Arc::new(webfetch::WebFetchTool::new()));
    registry.register(Arc::new(websearch::WebSearchTool::new()));
    registry.register(Arc::new(codesearch::CodeSearchTool::with_root(
        root_path.clone(),
    )));
    registry.register(Arc::new(todo::TodoReadTool::with_root(root_path.clone())));
    registry.register(Arc::new(skill::SkillTool::new()));
}
