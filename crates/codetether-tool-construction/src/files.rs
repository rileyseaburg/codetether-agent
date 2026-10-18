//! Delegate file construction to its dedicated catalog.
use crate::{Host, NativeTool};
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub(super) fn create(kind: ToolKind, host: &impl Host) -> Option<Arc<dyn Tool>> {
    codetether_file_tool_catalog::create(
        kind,
        || host.create(NativeTool::Edit),
        || host.create(NativeTool::MultiEdit),
    )
}
