use super::*;
pub(super) async fn run(
    payload: &RemoteSubtaskPayload,
    working_dir: &PathBuf,
    provider: Arc<dyn crate::provider::Provider>,
    tool_defs: Vec<crate::provider::ToolDefinition>,
    tool_registry: Arc<crate::tool::ToolRegistry>,
    system_prompt: &str,
    user_prompt: &str,
) -> codetether_swarm_core::agent_retry::AgentOutcome {
    run_agent_loop(
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
