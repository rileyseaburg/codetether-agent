//! Application registry and feature-sensitive tool bindings.
use codetether_application_registry::{Application, ToolRegistry};
use codetether_tool_core::Tool;
use std::marker::PhantomData;
use std::{path::PathBuf, sync::Arc};
/// Rooted registry factory retaining the application catalog and native tools.
/// # Examples
/// Use WorkspaceHost<T> with codetether_workspace_tools::composition.
pub type WorkspaceHost<T> =
    codetether_application_registry::workspace_factory::WorkspaceFactory<T, WorkspaceTools<T>>;
/// Rooted LSP and feature-gated scripting tools for application T.
/// # Examples
/// WorkspaceHost<T> selects these tools when creating a rooted registry.
pub struct WorkspaceTools<T>(PhantomData<fn() -> T>);
impl<T: Application> codetether_workspace_tools::Host for WorkspaceTools<T> {
    type Registry = ToolRegistry<T>;
    fn lsp(root_uri: String) -> Arc<dyn Tool> {
        Arc::new(codetether_native_lsp::tool::LspTool::with_root(root_uri))
    }
    fn scripting(root: PathBuf) -> Option<Arc<dyn Tool>> {
        #[cfg(feature = "tetherscript")]
        {
            Some(Arc::new(
                codetether_native_tetherscript::TetherScriptPluginTool::with_root(root),
            ))
        }
        #[cfg(not(feature = "tetherscript"))]
        {
            let _ = root;
            None
        }
    }
}
