//! Integration-tool construction with application identity and policy bindings.
use crate::NativeTool;
use codetether_tool_core::Tool;
use std::sync::Arc;

/// Concrete integration tools selected by the application catalog.
/// # Implementors
/// Applications retain the existing MCP authorization, RustyRoad workspace
/// policy and optional TetherScript host through their chosen tool types.
pub trait Types {
    /// MCP bridge with application authorization.
    type McpBridge: Tool + Default + 'static;
    /// RustyRoad integration with application workspace policy.
    type RustyRoad: Tool + Default + 'static;
    /// Optional TetherScript runtime with application policy.
    #[cfg(feature = "tetherscript")]
    type TetherScript: Tool + Default + 'static;
}

/// Construct the integration identified by the ordered catalog.
/// # Arguments
/// * `kind` - Integration kind requested by the native factory group.
/// # Returns
/// The selected default tool, or `None` for a kind handled by another group.
/// # Examples
/// ```no_run
/// # fn example<T: codetether_tool_construction::application::integrations::Types>() {
/// let tool = codetether_tool_construction::application::integrations::create::<T>(
///     codetether_tool_construction::NativeTool::McpBridge);
/// assert!(tool.is_some());
/// # }
/// ```
pub fn create<T: Types>(kind: NativeTool) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        NativeTool::McpBridge => Arc::new(T::McpBridge::default()) as Arc<dyn Tool>,
        NativeTool::RustyRoad => Arc::new(T::RustyRoad::default()) as Arc<dyn Tool>,
        #[cfg(feature = "tetherscript")]
        NativeTool::TetherScript => Arc::new(T::TetherScript::default()) as Arc<dyn Tool>,
        _ => return None,
    })
}
