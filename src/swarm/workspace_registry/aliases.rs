//! Workspace-rooted compatibility aliases.

use crate::tool::{ToolRegistry, alias::AliasTool, patch, todo};
use std::{path::Path, sync::Arc};

pub(super) fn install(registry: &mut ToolRegistry, root: &Path) {
    registry.register(Arc::new(AliasTool::new(
        "patch",
        Arc::new(patch::ApplyPatchTool::with_root(root.to_path_buf())),
    )));
    registry.register(Arc::new(AliasTool::new(
        "todo_read",
        Arc::new(todo::TodoReadTool::with_root(root.to_path_buf())),
    )));
    registry.register(Arc::new(AliasTool::new(
        "todo_write",
        Arc::new(todo::TodoWriteTool::with_root(root.to_path_buf())),
    )));
}
