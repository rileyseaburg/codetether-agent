//! Bind remote process sequencing to resources and shared agent execution.
use super::{RemoteHost, invoke, resources};
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::{agent_retry::AgentOutcome, remote_protocol::RemoteSubtaskPayload};
use codetether_swarm_orchestrator::Host as Providers;
use std::{marker::PhantomData, path::PathBuf, sync::Arc};
pub(super) struct Adapter<H>(PhantomData<fn() -> H>);
impl<H: RemoteHost> codetether_swarm_remote_runtime::Host for Adapter<H> {
    type Registry = <H::Providers as Providers>::Providers;
    type Tools = H::Registry;
    async fn resolve(
        payload: &RemoteSubtaskPayload,
        working_dir: &PathBuf,
    ) -> anyhow::Result<codetether_swarm_remote_runtime::Resources<Self::Registry, Self::Tools>>
    {
        resources::resolve::<H>(payload, working_dir).await
    }
    fn append_session_instructions(prompt: String) -> String {
        H::append_session_instructions(prompt)
    }
    fn invoke(
        payload: &RemoteSubtaskPayload,
        working_dir: &PathBuf,
        provider: Arc<dyn Provider>,
        tool_defs: Vec<ToolDefinition>,
        tool_registry: Arc<Self::Tools>,
        system_prompt: &str,
        user_prompt: &str,
    ) -> impl std::future::Future<Output = AgentOutcome> + Send {
        invoke::run::<H>(
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
