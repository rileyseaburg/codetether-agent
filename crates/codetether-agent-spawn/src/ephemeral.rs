//! Persistence-free, synchronous execution for ephemeral child agents.

use crate::EphemeralHost;
use anyhow::Result;
use codetether_agent_input::SpawnRequest;
use codetether_collaboration_state::execution as execution_state;
use codetether_tool_core::ToolResult;

/// Execute a synchronous one-shot child under the shared execution guard.
/// # Arguments
/// * `request` — Validated child input; detached execution is rejected.
/// * `warning` — Optional advisory model warning included in the result.
/// # Returns
/// The existing one-shot result with retained-checkout handoff when applicable.
/// # Errors
/// Propagates provider and workspace setup errors; loop errors become tool failures.
/// # Examples
/// Invoke with an application [EphemeralHost] implementation.
pub async fn run<H: EphemeralHost>(
    request: &SpawnRequest<'_>,
    warning: Option<&str>,
) -> Result<ToolResult> {
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
    let setup = super::ephemeral_setup::prepare::<H>(request).await?;
    let outcome = H::execute(setup.run, request.instructions, runtime_id).await;
    let result = super::ephemeral_result::build(request.name, outcome, warning);
    Ok(match setup.handoff {
        Some(handoff) => handoff.attach(result),
        None => result,
    })
}
