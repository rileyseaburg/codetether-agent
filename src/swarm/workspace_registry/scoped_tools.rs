//! Registration of tools that retain workspace state internally.

use super::{
    Capability, aliases, file_uri, scoped_bash::ScopedBashTool, scoped_git::ScopedGitTool,
};
use crate::tool::{ToolRegistry, codesearch, file, lsp, patch, todo};
use std::{path::Path, sync::Arc};

pub(super) fn install(registry: &mut ToolRegistry, root: &Path, capability: Capability) {
    let path = root.to_path_buf();
    registry.register(Arc::new(ScopedBashTool::new(path.clone())));
    registry.register(Arc::new(ScopedGitTool::new(
        path.clone(),
        capability.allows_commit(),
    )));
    registry.register(Arc::new(file::GlobTool::with_root(path.clone())));
    registry.register(Arc::new(lsp::LspTool::with_root(file_uri::build(root))));
    registry.register(Arc::new(codesearch::CodeSearchTool::with_root(
        path.clone(),
    )));
    registry.register(Arc::new(patch::ApplyPatchTool::with_root(path.clone())));
    registry.register(Arc::new(todo::TodoReadTool::with_root(path.clone())));
    registry.register(Arc::new(todo::TodoWriteTool::with_root(path.clone())));
    #[cfg(feature = "tetherscript")]
    registry.register(Arc::new(
        crate::tool::tetherscript::TetherScriptPluginTool::with_root(path),
    ));
    aliases::install(registry, root);
}
