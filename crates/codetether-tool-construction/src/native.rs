//! Contracts for constructors requiring application session or provider types.
use codetether_tool_core::Tool;
use std::sync::Arc;
/// A constructor whose concrete type depends on application runtime bindings.
///
/// Variants identify editor, context, runtime and integration constructors.
/// # Examples
/// ```
/// use codetether_tool_construction::NativeTool;
/// assert!(matches!(NativeTool::Edit, NativeTool::Edit));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum NativeTool {
    /// Construct the application-bound Edit tool.
    Edit,
    /// Construct the application-bound MultiEdit tool.
    MultiEdit,
    /// Construct the application-bound Lsp tool.
    Lsp,
    /// Construct the application-bound ContextBrowse tool.
    ContextBrowse,
    /// Construct the application-bound ContextBudget tool.
    ContextBudget,
    /// Construct the application-bound ContextPin tool.
    ContextPin,
    /// Construct the application-bound ContextSummarizeCached tool.
    ContextSummarizeCached,
    /// Construct the application-bound Memory tool.
    Memory,
    /// Construct the application-bound RalphDefault tool.
    RalphDefault,
    /// Construct the application-bound Undo tool.
    Undo,
    /// Construct the application-bound SwarmExecute tool.
    SwarmExecute,
    /// Construct the application-bound Go tool.
    Go,
    /// Construct the application-bound McpBridge tool.
    McpBridge,
    /// Construct the application-bound RustyRoad tool.
    RustyRoad,
    #[cfg(feature = "tetherscript")]
    /// Construct the application-bound TetherScript tool.
    TetherScript,
}
/// Supplies concrete constructors bound to application state and policies.
///
/// # Implementors
/// Application adapters implement this trait using their session and provider
/// types. Implementations construct only the requested tool.
pub trait Host {
    /// Construct the requested application-bound tool instance.
    fn create(&self, kind: NativeTool) -> Arc<dyn Tool>;
}
