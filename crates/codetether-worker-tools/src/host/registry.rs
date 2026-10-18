//! Executable registry allocation and registration.
use codetether_tool_core::Tool;
use std::sync::Arc;
/// Application registry used to execute the assembled tools.
/// # Implementors
/// Adapters forward to their existing empty-registry constructor and insertion.
pub trait Registry {
    /// Concrete application registry.
    type Registry;
    /// Create an empty registry, before any worker groups are added.
    fn new_registry() -> Self::Registry;
    /// Register one tool in call order.
    fn register(registry: &mut Self::Registry, tool: Arc<dyn Tool>);
}
