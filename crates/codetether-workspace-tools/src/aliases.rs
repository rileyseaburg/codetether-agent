//! Workspace-rooted compatibility aliases.

use crate::Registry;
use codetether_patch as patch;
use codetether_todo as todo;
use codetether_tool_core::AliasTool;
use std::{path::Path, sync::Arc};

pub(crate) fn install(registry: &mut impl Registry, root: &Path) {
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
