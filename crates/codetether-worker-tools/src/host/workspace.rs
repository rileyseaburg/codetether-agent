//! Session-aware tool constructors used by workspace groups.
use codetether_tool_core::Tool;
use std::sync::Arc;
/// Constructors requiring application session or provider adapters.
/// # Implementors
/// The worker's native LSP, multi-edit and undo bindings.
pub trait Workspace {
    /// Construct an LSP tool for the supplied file URI.
    fn lsp(root: String) -> Arc<dyn Tool>;
    /// Construct the application's multi-edit tool.
    fn multiedit() -> Arc<dyn Tool>;
    /// Construct undo with the application's policy.
    fn undo() -> Arc<dyn Tool>;
}
