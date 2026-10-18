//! Native registry operations for shared autonomous filtering.
use crate::tool::ToolRegistry;
/// Apply the shared autonomous/read-only restriction policy.
pub fn restrict_registry(registry: &mut ToolRegistry, read_only: bool) {
    codetether_swarm_core::tool_policy::restrict_registry(registry, read_only);
}
