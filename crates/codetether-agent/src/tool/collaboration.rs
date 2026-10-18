//! Codex-compatible first-class collaboration tool registration.

#[path = "collaboration/context.rs"]
mod context;
#[cfg(test)]
#[path = "collaboration/list.rs"]
mod list;
#[cfg(test)]
#[path = "collaboration/send_input.rs"]
mod send_input;
#[cfg(test)]
#[path = "collaboration/spawn.rs"]
mod spawn;
#[cfg(test)]
#[path = "collaboration/wait.rs"]
mod wait;

mod host {
    codetether_collaboration_tools::declare_application_host!(
        crate::tool::agent::AgentTool;
        crate::tool::agent::collaboration_runtime::agent_tree::canonical,
        crate::tool::agent::residency::open,
        crate::tool::collaboration::context::resume_config,
        crate::tool::agent::communication::steer,
        crate::tool::agent::communication::queue_only,
        crate::tool::agent::collaboration_runtime::agent_tree::list,
        crate::tool::agent::bridge::find_agent_tool_agent_for_parent,
        crate::tool::agent::collaboration_runtime::thread_status::subscribe,
        crate::tool::agent::collaboration_runtime::thread_status::set,
        crate::tool::agent::collaboration_runtime::parent_activity::until,
    );
}

use super::ToolRegistry;
use codetether_tool_registry::catalog::ToolKind;
codetether_collaboration_tools::declare_catalog!(
    codetether_tool_registry::catalog::ToolKind,
    crate::tool::agent::AgentTool,
    crate::tool::collaboration::host::ApplicationHost,
);

/// Register the legacy agent tool and first-class collaboration operations.
pub fn register(registry: &mut ToolRegistry) {
    super::catalog::register_group(registry, ToolKind::CollaborationGroup);
}

#[cfg(test)]
#[path = "collaboration/tests.rs"]
mod tests;
