//! Native constructor bindings for the shared ordered tool catalog.
use super::*;
use codetether_tool_registry::catalog::{Host, ToolKind};
mod bindings;
mod construct;
mod provider;
pub(super) type Native<'a> =
    codetether_application_registry::catalog::Catalog<'a, construct::Constructors>;
pub(super) fn register_group(registry: &mut ToolRegistry, kind: ToolKind) {
    Native::new(registry, None).register(kind);
}
