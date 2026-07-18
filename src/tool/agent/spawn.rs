//! Spawn orchestration for the agent tool.
//!
//! Coordinates spawn request parsing, validation, session creation, and the
//! durable/ephemeral persistence policy. After persistence succeeds, the first
//! model turn is auto-started via [`super::spawn_run`] so the sub-agent begins
//! working immediately instead of sitting idle (issue #295).

use super::params::Params;
use super::spawn_request::SpawnRequest;
use crate::tool::ToolResult;
use anyhow::Result;

#[path = "spawn/durable.rs"]
mod durable;
#[path = "spawn/ephemeral.rs"]
mod ephemeral;
#[path = "spawn/ephemeral_result.rs"]
mod ephemeral_result;
#[path = "spawn/ephemeral_setup.rs"]
mod ephemeral_setup;
#[path = "spawn/finish.rs"]
mod finish;
#[path = "spawn/identity.rs"]
mod identity;
#[path = "spawn/prepare.rs"]
mod prepare;
#[path = "spawn/session.rs"]
mod session;

/// Spawns a new sub-agent and auto-starts its first turn.
pub(super) async fn handle_spawn(params: &Params) -> Result<ToolResult> {
    let request = SpawnRequest::from_params(params)?;
    let prepared = match prepare::run(&request).await {
        Ok(prepared) => prepared,
        Err(result) => return Ok(result),
    };
    if request.ephemeral {
        return ephemeral::run(&request, prepared.warning()).await;
    }
    durable::run(params, &request, prepared).await
}
