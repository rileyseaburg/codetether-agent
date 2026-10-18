//! Native constructor groups supplied by an embedding application.
use crate::NativeTool;
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;

/// Binds concrete tools without owning constructor search order.
/// # Implementors
/// Applications return a tool only for kinds owned by each capability group.
pub trait Factories {
    /// Construct an application-bound file editor, or decline the kind.
    fn files(&self, kind: NativeTool) -> Option<Arc<dyn Tool>>;
    /// Construct an application-bound context tool, or decline the kind.
    fn context(&self, kind: NativeTool) -> Option<Arc<dyn Tool>>;
    /// Construct an application-bound execution tool, or decline the kind.
    fn execution(&self, kind: NativeTool) -> Option<Arc<dyn Tool>>;
    /// Construct an application-bound integration tool, or decline the kind.
    fn integrations(&self, kind: NativeTool) -> Option<Arc<dyn Tool>>;
    /// Construct a goal capability after shared construction declines it.
    fn goals(&self, kind: ToolKind) -> Option<Arc<dyn Tool>>;
    /// Construct a collaboration capability after goal construction declines it.
    fn collaboration(&self, kind: ToolKind) -> Option<Arc<dyn Tool>>;
}
