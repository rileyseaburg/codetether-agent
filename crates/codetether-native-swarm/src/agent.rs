//! Native runtime policy and parent-goal accounting for shared sub-agents.
use codetether_application_registry::{Application, ToolRegistry};
use codetether_tool_core::Tool;
use serde_json::Value;
use std::marker::PhantomData;
use std::sync::Arc;
/// Native agent services for tool application T.
/// # Examples
/// Use AgentHost<T> with the shared swarm agent loop and the same registry T.
pub struct AgentHost<T>(PhantomData<fn() -> T>);
impl<T: Application> codetether_swarm_agent::Host for AgentHost<T> {
    type Registry = ToolRegistry<T>;
    fn get(registry: &ToolRegistry<T>, name: &str) -> Option<Arc<dyn Tool>> {
        registry.get(name)
    }
}
impl<T: Application> codetether_provider_step::controlled::Accounting for AgentHost<T> {
    async fn record(
        session_id: &str,
        tokens: usize,
        elapsed: std::time::Duration,
    ) -> anyhow::Result<()> {
        codetether_native_goals::runtime::record_usage(session_id, tokens, elapsed).await
    }
}
impl<T: Application> codetether_swarm_core::agent_loop::tools::input::Policy for AgentHost<T> {
    async fn denial(name: &str, args: &Value) -> Option<String> {
        crate::policy::runtime_denial(name, args).await
    }
}

impl<T: Application> codetether_swarm_agent::job::JobHost for AgentHost<T> {
    type Workspace = crate::workspace::WorkspaceHost<T>;
    fn system_prompt(input: codetether_swarm_core::prompt_policy::SystemPromptInput<'_>) -> String {
        crate::policy::system_prompt(input)
    }
}
impl<T: Application> codetether_swarm_agent::remote::RemoteHost for AgentHost<T> {
    type Providers = crate::Providers;
    fn append_session_instructions(prompt: String) -> String {
        codetether_native_goals::turn_instructions::append(prompt)
    }
}
