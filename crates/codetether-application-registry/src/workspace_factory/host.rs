//! Delegate rooted tool construction to the application's workspace host.
use super::WorkspaceFactory;
use crate::{Application, ToolRegistry};
use codetether_tool_core::Tool;
use codetether_workspace_tools::Host;
use std::{path::PathBuf, sync::Arc};
impl<A: Application, W: Host<Registry = ToolRegistry<A>>> Host for WorkspaceFactory<A, W> {
    type Registry = ToolRegistry<A>;
    fn lsp(root_uri: String) -> Arc<dyn Tool> {
        W::lsp(root_uri)
    }
    fn scripting(root: PathBuf) -> Option<Arc<dyn Tool>> {
        W::scripting(root)
    }
}
