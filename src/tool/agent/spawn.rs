//! Spawn orchestration for the agent tool.
//!
//! Coordinates spawn request parsing, validation, session creation, and the
//! durable/ephemeral persistence policy. After persistence succeeds, the first
//! model turn is auto-started via [`super::spawn_run`] so the sub-agent begins
//! working immediately instead of sitting idle (issue #295).

use super::params::Params;
use crate::tool::ToolResult;
use anyhow::Result;

#[path = "spawn/ephemeral.rs"]
mod ephemeral;
#[path = "spawn/ephemeral_result.rs"]
mod ephemeral_result;
#[path = "spawn/ephemeral_setup.rs"]
mod ephemeral_setup;
#[path = "spawn/host.rs"]
mod host;
#[path = "spawn/identity.rs"]
mod identity;
#[path = "spawn/session.rs"]
mod session;
#[path = "spawn/session_workspace.rs"]
mod session_workspace;
#[path = "spawn/workspace.rs"]
mod workspace;
/// Spawn through shared orchestration using native runtime services.
pub(super) async fn handle_spawn(params: &Params) -> Result<ToolResult> {
    codetether_agent_spawn::handle_spawn::<host::ApplicationHost>(params).await
}
