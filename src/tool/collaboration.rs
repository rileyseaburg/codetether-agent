//! Codex-compatible first-class collaboration tool registration.

#[path = "collaboration/context.rs"]
mod context;
#[path = "collaboration/ensure.rs"]
mod ensure;
#[path = "collaboration/close.rs"]
mod close;
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

use super::ToolRegistry;
use std::sync::Arc;

/// Register the legacy agent tool and first-class collaboration operations.
pub fn register(registry: &mut ToolRegistry) {
    registry.register(Arc::new(super::agent::AgentTool::new()));
    registry.register(Arc::new(spawn::SpawnAgentTool));
    registry.register(Arc::new(followup::FollowupTaskTool));
    registry.register(Arc::new(list::ListAgentsTool));
    registry.register(Arc::new(wait::WaitAgentTool));
    registry.register(Arc::new(interrupt::InterruptAgentTool));
    registry.register(Arc::new(close::CloseAgentTool));
    registry.register(Arc::new(resume::ResumeAgentTool));
    registry.register(Arc::new(send_input::SendInputTool));
    registry.register(Arc::new(send_message::SendMessageTool));
}

#[cfg(test)]
#[path = "collaboration/tests.rs"]
mod tests;
