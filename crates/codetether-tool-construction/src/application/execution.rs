//! Execution-tool selection with application-bound default constructors.
use crate::NativeTool;
use codetether_tool_core::Tool;
use std::sync::Arc;

/// Construct the requested execution tool using its application type.
/// # Arguments
/// * `kind` - Ordered-catalog identity to construct.
/// # Returns
/// A default Ralph, undo, swarm or go tool, or `None` for other kinds.
/// # Examples
/// ```no_run
/// # fn example<R, U, S, G>() where
/// # R: codetether_tool_core::Tool + Default + 'static,
/// # U: codetether_tool_core::Tool + Default + 'static,
/// # S: codetether_tool_core::Tool + Default + 'static,
/// # G: codetether_tool_core::Tool + Default + 'static {
/// let tool = codetether_tool_construction::application::execution::create::<R, U, S, G>(
///     codetether_tool_construction::NativeTool::Go);
/// assert!(tool.is_some());
/// # }
/// ```
pub fn create<R, U, S, G>(kind: NativeTool) -> Option<Arc<dyn Tool>>
where
    R: Tool + Default + 'static,
    U: Tool + Default + 'static,
    S: Tool + Default + 'static,
    G: Tool + Default + 'static,
{
    Some(match kind {
        NativeTool::RalphDefault => Arc::new(R::default()) as Arc<dyn Tool>,
        NativeTool::Undo => Arc::new(U::default()) as Arc<dyn Tool>,
        NativeTool::SwarmExecute => Arc::new(S::default()) as Arc<dyn Tool>,
        NativeTool::Go => Arc::new(G::default()) as Arc<dyn Tool>,
        _ => return None,
    })
}
