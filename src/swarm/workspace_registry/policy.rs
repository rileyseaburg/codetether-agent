//! Capability filtering for workspace-scoped swarm tools.

use super::Capability;
use crate::tool::{ToolRegistry, readonly};

const UNSCOPED: &[&str] = &[
    "question",
    "confirm_edit",
    "confirm_multiedit",
    "plan_enter",
    "plan_exit",
    "agent",
    "swarm_execute",
    "relay_autochat",
    "ralph",
    "prd",
    "go",
    "undo",
    "kubernetes",
    "k8s_tool",
    "tetherscript_plugin",
    "diff",
];

pub(super) fn apply(registry: &mut ToolRegistry, capability: Capability) {
    for id in UNSCOPED {
        registry.unregister(id);
    }
    if capability == Capability::Mutating {
        return;
    }
    let ids = registry
        .list()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    for id in ids {
        if !allowed(&id, capability) {
            registry.unregister(&id);
        }
    }
}

fn allowed(id: &str, capability: Capability) -> bool {
    let inspect = readonly::is_read_only(id)
        || matches!(id, "file_info" | "head_tail" | "todoread" | "todo_read");
    inspect || (capability == Capability::Verification && matches!(id, "bash" | "git"))
}
