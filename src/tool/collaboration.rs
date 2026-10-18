//! Codex-compatible first-class collaboration tool registration.

#[path = "collaboration/close.rs"]
mod close;
#[path = "collaboration/context.rs"]
mod context;
#[path = "collaboration/ensure.rs"]
mod ensure;
#[path = "collaboration/followup.rs"]
mod followup;
#[path = "collaboration/interrupt.rs"]
mod interrupt;
#[path = "collaboration/legacy.rs"]
mod legacy;
#[path = "collaboration/list.rs"]
mod list;
#[path = "collaboration/resume.rs"]
mod resume;
#[path = "collaboration/send_input.rs"]
mod send_input;
#[path = "collaboration/send_message.rs"]
mod send_message;
#[path = "collaboration/spawn.rs"]
mod spawn;
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
#[path = "collaboration/catalog_construct.rs"]
mod catalog_construct;
pub(crate) use catalog_construct::construct;

/// Register the legacy agent tool and first-class collaboration operations.
pub fn register(registry: &mut ToolRegistry) {
    super::catalog::register_group(registry, ToolKind::CollaborationGroup);
}

#[cfg(test)]
#[path = "collaboration/tests.rs"]
mod tests;
