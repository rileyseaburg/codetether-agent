//! Native registry operations for shared autonomous filtering.
use codetether_application_registry::{Application, ToolRegistry};
/// Apply the shared autonomous/read-only restriction policy.
pub fn restrict_registry<T: Application>(registry: &mut ToolRegistry<T>, read_only: bool) {
    codetether_swarm_core::tool_policy::restrict_registry(registry, read_only);
}
