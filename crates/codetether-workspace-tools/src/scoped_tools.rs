//! Registration of tools that retain workspace state internally.

use super::{
    Capability, aliases, file_uri, scoped_bash::ScopedBashTool, scoped_git::ScopedGitTool,
};
use crate::{Host, Registry};
use codetether_code_search as codesearch;
use codetether_file_tools as file;
use codetether_patch as patch;
use codetether_todo as todo;
use std::{path::Path, sync::Arc};

pub(crate) fn install<H: Host>(registry: &mut H::Registry, root: &Path, capability: Capability) {
    let path = root.to_path_buf();
    if capability == Capability::Verification {
        registry.register(Arc::new(super::readonly_bash::ReadOnlyBashTool::new(
            path.clone(),
        )));
    } else {
        registry.register(Arc::new(ScopedBashTool::new(path.clone())));
    }
    registry.register(Arc::new(ScopedGitTool::new(
        path.clone(),
        capability.allows_commit(),
    )));
    registry.register(Arc::new(file::GlobTool::with_root(path.clone())));
    registry.register(H::lsp(file_uri::build(root)));
    registry.register(Arc::new(codesearch::CodeSearchTool::with_root(
        path.clone(),
    )));
    registry.register(Arc::new(patch::ApplyPatchTool::with_root(path.clone())));
    registry.register(Arc::new(todo::TodoReadTool::with_root(path.clone())));
    registry.register(Arc::new(todo::TodoWriteTool::with_root(path.clone())));
    if let Some(tool) = H::scripting(path) {
        registry.register(tool);
    }
    aliases::install(registry, root);
}
