//! Native resource bindings for the remote swarm process lifecycle.
use crate::cli::SwarmSubagentArgs;
use anyhow::Result;
/// Execute a remote task from command-line/environment configuration.
/// # Arguments
/// * `args` — Payload source and remote process options.
/// # Returns
/// Success after emitting the structured task result.
/// # Errors
/// Returns payload, workspace, provider, execution, or serialization errors.
/// # Examples
/// Invoked by the `swarm-subagent` CLI command.
pub async fn run_swarm_subagent(args: SwarmSubagentArgs) -> Result<()> {
    codetether_swarm_agent::remote::run::<super::executor::LoopHost>(
        args.payload_base64,
        args.payload_env,
    )
    .await
}
