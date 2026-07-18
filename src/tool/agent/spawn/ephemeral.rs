//! Persistence-free, synchronous execution for ephemeral child agents.

use super::super::{execution_state, spawn_request::SpawnRequest};
use crate::swarm::executor::run_agent_loop;
use crate::tool::ToolResult;
use anyhow::Result;

pub(super) async fn run(request: &SpawnRequest<'_>, warning: Option<&str>) -> Result<ToolResult> {
    if request.detach {
        return Ok(ToolResult::error(
            "ephemeral agents cannot detach; omit detach or set it to false",
        ));
    }
    let runtime_id = format!("ephemeral-{}", uuid::Uuid::new_v4());
    let Some(_guard) = execution_state::try_start(&runtime_id) else {
        return Ok(ToolResult::error(format!(
            "Agent @{} is busy",
            request.name
        )));
    };
    let setup = super::ephemeral_setup::prepare(request).await?;
    let outcome = run_agent_loop(
        setup.provider,
        &setup.model,
        &setup.prompt,
        request.instructions,
        setup.registry.definitions(),
        setup.registry,
        crate::session::DEFAULT_MAX_STEPS,
        300,
        None,
        runtime_id,
        None,
        Some(setup.workspace),
    )
    .await;
    Ok(super::ephemeral_result::build(
        request.name,
        outcome,
        warning,
    ))
}
