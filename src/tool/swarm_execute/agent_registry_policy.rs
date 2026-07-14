//! Tool filtering policy for autonomous direct-swarm workers.

use crate::tool::ToolRegistry;

const DENYLIST: &[&str] = &[
    "question",
    "confirm_edit",
    "confirm_multiedit",
    "plan_enter",
    "plan_exit",
    "swarm_execute",
    "agent",
];

pub(super) fn apply(registry: &mut ToolRegistry, read_only: bool) {
    crate::swarm::tool_policy::restrict_registry(registry, read_only);
    for tool in DENYLIST {
        registry.unregister(tool);
    }
}
