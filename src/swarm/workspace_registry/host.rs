//! Application registry and feature-sensitive tool bindings.
use crate::tool::{Tool, ToolRegistry};
use std::{path::PathBuf, sync::Arc};
pub(crate) struct Native;
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
