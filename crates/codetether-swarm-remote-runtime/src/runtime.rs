//! Ordered remote task execution and final result emission.
use crate::{Host, setup};
use anyhow::Result;
use codetether_swarm_core::remote_protocol::{
    SWARM_SUBTASK_RESULT_PREFIX, from_agent_result, prompts,
};
use std::time::Instant;
/// Execute one remote task from command-line/environment configuration.
///
/// # Arguments
/// * `payload_base64` - Explicit encoded payload, when supplied.
/// * `payload_env` - Environment variable used when no explicit payload is supplied.
///
/// # Returns
/// Success after emitting the structured task result.
///
/// # Errors
/// Returns setup/provider errors or an error serializing the final result.
///
/// # Examples
/// This entrypoint is invoked by the `swarm-subagent` CLI command.
pub async fn run<H: Host>(payload_base64: Option<String>, payload_env: String) -> Result<()> {
    let payload = setup::payload(payload_base64, payload_env)?;
    let working_dir = setup::workspace(&payload)?;
    let resources = H::resolve(&payload, &working_dir).await?;
    let (system_prompt, user_prompt) =
        prompts::build(&payload, &working_dir, H::append_session_instructions);
    let probe =
        codetether_swarm_k8s::probe::start(&payload.subtask_id, payload.probe_interval_secs);
    let started = Instant::now();
    let run_result = H::invoke(
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
