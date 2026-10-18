//! Ordered application constructor groups and capability fallbacks.
//!
//! Implement [Factories] to bind concrete application types. [create] uses the
//! existing shared constructors, then goal and collaboration capability groups.
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub mod execution;
mod host;
pub mod integrations;
mod native;
pub use host::Factories;

/// Construct a catalog tool using the application capability groups.
/// # Arguments
/// * `kind` — Catalog identity requested by registration.
/// * `factories` — Concrete application constructors grouped by responsibility.
/// # Returns
/// The constructed tool instance.
/// # Panics
/// Panics with the existing constructor failure when no group handles the kind.
/// # Examples
/// ```no_run
/// # fn example(factories: &impl codetether_tool_construction::application::Factories) {
/// let _tool = codetether_tool_construction::application::create(
///     codetether_tool_registry::catalog::ToolKind::Read, factories,
/// );
/// # }
/// ```
pub fn create(kind: ToolKind, factories: &impl Factories) -> Arc<dyn Tool> {
    crate::create(kind, &native::Adapter(factories))
        .or_else(|| factories.goals(kind))
        .or_else(|| factories.collaboration(kind))
        .expect("native catalog constructor")
}
