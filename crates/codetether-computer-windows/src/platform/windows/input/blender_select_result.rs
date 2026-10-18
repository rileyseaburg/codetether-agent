//! Native cursor/window enrichment for shared Blender result assembly.
use serde_json::Value;
/// Build the verified selection result with native mouse/window metadata.
pub fn build(name: &str, details: Value) -> codetether_tool_core::ToolResult {
    codetether_computer_contract::blender_result::build(name, details, super::report::mouse_result)
}
