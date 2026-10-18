//! Native resource bindings for the remote swarm process lifecycle.
use super::executor::{run_agent_loop, workspace_registry};
use crate::{cli::SwarmSubagentArgs, provider::ProviderRegistry};
use anyhow::{Result, anyhow};
use codetether_swarm_core::remote_protocol::RemoteSubtaskPayload;
use std::{path::PathBuf, sync::Arc};
mod host;
mod invoke;
mod resources;
struct Native;
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
    codetether_swarm_remote_runtime::run::<Native>(args.payload_base64, args.payload_env).await
}
