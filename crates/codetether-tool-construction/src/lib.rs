//! Tool construction grouped by context, execution, media, search and integrations.
//!
//! [create] resolves scalar catalog kinds, using [Host] only for the concrete
//! types that require application bindings. File construction remains owned by
//! its dedicated catalog. Registration order and aliases belong to the registry.
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
mod context;
mod execution;
mod files;
mod integrations;
mod media;
mod native;
mod search_tools;
pub use native::{Host, NativeTool};

/// Construct a scalar tool using its shared implementation or application factory.
///
/// # Arguments
/// * `kind` — Scalar identity from the ordered tool registry.
/// * `host` — Application-bound constructors requested only as needed.
/// # Returns
/// A tool, or `None` for kinds owned by other catalogs or registration groups.
/// # Examples
/// ```no_run
/// # fn setup(host: &impl codetether_tool_construction::Host) {
/// use codetether_tool_registry::catalog::ToolKind;
/// assert!(codetether_tool_construction::create(ToolKind::Read, host).is_some());
/// # }
/// ```
pub fn create(kind: ToolKind, host: &impl Host) -> Option<Arc<dyn Tool>> {
    files::create(kind, host)
        .or_else(|| context::create(kind, host))
        .or_else(|| execution::create(kind, host))
        .or_else(|| media::create(kind))
        .or_else(|| search_tools::create(kind))
        .or_else(|| integrations::create(kind, host))
}
