//! Map remote payload limits and identity to the shared agent loop.
use super::RemoteHost;
use crate::run_agent_loop;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::{agent_retry::AgentOutcome, remote_protocol::RemoteSubtaskPayload};
use std::{path::PathBuf, sync::Arc};
pub(super) async fn run<H: RemoteHost>(
    payload: &RemoteSubtaskPayload,
    working_dir: &PathBuf,
    provider: Arc<dyn Provider>,
    tool_defs: Vec<ToolDefinition>,
    tool_registry: Arc<H::Registry>,
    system_prompt: &str,
    user_prompt: &str,
) -> AgentOutcome {
    run_agent_loop::<H>(
        provider,
        &payload.model,
        system_prompt,
        user_prompt,
        tool_defs,
        tool_registry,
        payload.max_steps,
        payload.timeout_secs,
        None,
        payload.subtask_id.clone(),
        None,
        Some(working_dir.clone()),
    )
    .await
}
