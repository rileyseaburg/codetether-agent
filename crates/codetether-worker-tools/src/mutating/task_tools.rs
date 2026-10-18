//! Register non-provider workflow tools.
use crate::Host;
use std::sync::Arc;
/// Register non-provider workflow tools.
///
/// # Arguments
/// * `registry` — Registry receiving workflow and MCP tools.
/// * `H` — Application registry and constructor bindings.
/// # Returns
/// Registers this group in the supplied registry.
/// # Examples
/// See [crate] for a generic application-bound construction example.
pub fn register_task_tools<H: Host>(registry: &mut H::Registry) {
    H::register(registry, Arc::new(codetether_task_tool::TaskTool::new()));
    H::register(registry, Arc::new(codetether_plan::PlanEnterTool::new()));
    H::register(registry, Arc::new(codetether_plan::PlanExitTool::new()));
    H::register(registry, Arc::new(codetether_prd_tool::PrdTool::new()));
    H::register(registry, H::mcp_bridge());
}
