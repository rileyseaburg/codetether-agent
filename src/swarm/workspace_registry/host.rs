//! Application registry and feature-sensitive tool bindings.
use crate::tool::{Tool, ToolRegistry};
use std::{path::PathBuf, sync::Arc};
pub(super) struct Native;
impl codetether_workspace_tools::Registry for ToolRegistry {
    fn register(&mut self, tool: Arc<dyn Tool>) {
        ToolRegistry::register(self, tool);
    }
    fn unregister(&mut self, id: &str) {
        ToolRegistry::unregister(self, id);
    }
    fn list(&self) -> Vec<&str> {
        ToolRegistry::list(self)
    }
}
impl codetether_workspace_tools::Host for Native {
    type Registry = ToolRegistry;
    fn lsp(root_uri: String) -> Arc<dyn Tool> {
        Arc::new(crate::tool::lsp::LspTool::with_root(root_uri))
    }
    fn scripting(root: PathBuf) -> Option<Arc<dyn Tool>> {
        #[cfg(feature = "tetherscript")]
        {
            Some(Arc::new(
                crate::tool::tetherscript::TetherScriptPluginTool::with_root(root),
            ))
        }
        #[cfg(not(feature = "tetherscript"))]
        {
            let _ = root;
            None
        }
    }
}
