//! Native constructor bindings for the shared ordered tool catalog.
use super::*;
use codetether_tool_registry::catalog::{Host, ToolKind};
mod construct;
mod context;
mod execution;
mod files;
mod host;
mod integrations;
mod provider;
pub(super) use host::Native;
pub(super) fn register_group(registry: &mut ToolRegistry, kind: ToolKind) {
    Native {
        registry,
        provider: None,
    }
    .register(kind);
}
