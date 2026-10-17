//! Remote task process entrypoint and application resource adapters.
//!
//! Setup, resources, prompts and invocation are separate; the shared protocol
//! owns result construction. Probe shutdown precedes structured result emission.
use super::executor::{run_agent_loop, workspace_registry};
use super::tool_policy;
use crate::{cli::SwarmSubagentArgs, provider::ProviderRegistry};
use anyhow::{Context, Result, anyhow};
use codetether_swarm_core::remote_protocol::{
    RemoteSubtaskPayload, SWARM_SUBTASK_RESULT_PREFIX, decode_payload, from_agent_result,
};
use std::{path::PathBuf, sync::Arc, time::Instant};
mod invoke;
mod prompts;
mod resources;
mod setup;
/// Execute one remote task from command-line/environment configuration.
///
/// # Arguments
/// * `args` - Payload source and remote process options.
///
/// # Returns
/// Success after emitting the structured task result.
///
/// # Errors
/// Returns setup/provider errors or an error serializing the final result.
///
/// # Examples
/// This entrypoint is invoked by the `swarm-subagent` CLI command.
pub async fn run_swarm_subagent(args: SwarmSubagentArgs) -> Result<()> {
    let payload = setup::payload(args)?;
    let working_dir = setup::workspace(&payload)?;
    let resources = resources::resolve(&payload, &working_dir).await?;
    let (system_prompt, user_prompt) = prompts::build(&payload, &working_dir);
    let probe = super::remote_probe::start(&payload.subtask_id, payload.probe_interval_secs);
    let started = Instant::now();
    let run_result = invoke::run(
        &payload,
        &working_dir,
        resources.provider,
        resources.tool_defs,
        resources.tool_registry,
        &system_prompt,
        &user_prompt,
    )
    .await;
    probe.finish();
    let result = from_agent_result(&payload, run_result, || {
        started.elapsed().as_millis() as u64
    });
    println!(
        "{}{}",
        SWARM_SUBTASK_RESULT_PREFIX,
        serde_json::to_string(&result)?
    );
    Ok(())
}
