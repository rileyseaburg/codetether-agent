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

#[path = "collaboration/host.rs"]
mod host;

#[path = "collaboration/messaging_host.rs"]
mod messaging_host;

#[path = "collaboration/wait_host.rs"]
mod wait_host;

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
