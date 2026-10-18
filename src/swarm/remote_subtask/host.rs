//! Provider/tool registry and session-instruction bindings.
use super::*;
impl codetether_swarm_remote_runtime::Host for Native {
    type Registry = ProviderRegistry;
    type Tools = crate::tool::ToolRegistry;
    async fn resolve(
        payload: &RemoteSubtaskPayload,
        working_dir: &PathBuf,
    ) -> Result<codetether_swarm_remote_runtime::Resources<Self::Registry, Self::Tools>> {
        resources::resolve(payload, working_dir).await
    }
    fn append_session_instructions(prompt: String) -> String {
        crate::session::tasks::turn_instructions::append(prompt)
    }
    fn invoke(
        payload: &RemoteSubtaskPayload,
        working_dir: &PathBuf,
        provider: Arc<dyn crate::provider::Provider>,
        tool_defs: Vec<crate::provider::ToolDefinition>,
        tool_registry: Arc<Self::Tools>,
        system_prompt: &str,
        user_prompt: &str,
    ) -> impl std::future::Future<Output = codetether_swarm_core::agent_retry::AgentOutcome> + Send
    {
        invoke::run(
            payload,
            working_dir,
            provider,
            tool_defs,
            tool_registry,
            system_prompt,
            user_prompt,
        )
    }
}
