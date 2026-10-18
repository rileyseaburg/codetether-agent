use codetether_tool_core::Tool;
use std::{path::PathBuf, sync::Arc};
/// Registry operations required to replace and filter workspace tools.
/// # Implementors
/// Application registries keep ownership and lookup semantics; this trait exposes
/// only operations needed by workspace installation and filtering.
pub trait Registry {
    /// Replace or add a tool by its canonical identifier.
    fn register(&mut self, tool: Arc<dyn Tool>);
    /// Remove an identifier if present.
    fn unregister(&mut self, id: &str);
    /// List current identifiers for capability filtering.
    fn list(&self) -> Vec<&str>;
}
/// Native constructors whose identity or feature availability is application-owned.
/// # Implementors
/// Applications supply their LSP client and optional scripting runtime bindings.
pub trait Host {
    /// Registry receiving rooted tool instances.
    type Registry: Registry;
    /// Create the native LSP tool for a workspace file URI.
    fn lsp(root_uri: String) -> Arc<dyn Tool>;
    /// Create the scripting tool when the application feature is enabled.
    fn scripting(root: PathBuf) -> Option<Arc<dyn Tool>>;
}
