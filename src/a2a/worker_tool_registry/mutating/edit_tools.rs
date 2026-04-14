use crate::tool::{
    ToolRegistry, advanced_edit, bash, confirm_edit, confirm_multiedit, file, multiedit, patch,
    todo, undo,
};
use std::path::PathBuf;
use std::sync::Arc;

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
    registry.register(Arc::new(file::WriteTool::new()));
    registry.register(Arc::new(advanced_edit::AdvancedEditTool::new()));
    registry.register(Arc::new(bash::BashTool::with_cwd(root_path.clone())));
    registry.register(Arc::new(multiedit::MultiEditTool::new()));
    registry.register(Arc::new(patch::ApplyPatchTool::with_root(
        root_path.clone(),
    )));
    registry.register(Arc::new(todo::TodoWriteTool::with_root(root_path)));
    registry.register(Arc::new(confirm_edit::ConfirmEditTool::new()));
    registry.register(Arc::new(confirm_multiedit::ConfirmMultiEditTool::new()));
    registry.register(Arc::new(undo::UndoTool));
}
