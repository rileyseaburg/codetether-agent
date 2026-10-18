//! Public native specialization of shared agent state ownership.
/// Agent with the application tool registry and session services.
/// # Examples
/// ```rust,no_run
/// # fn example(info: codetether_agent::agent::AgentInfo, provider: std::sync::Arc<dyn codetether_agent::provider::Provider>, tools: codetether_agent::tool::ToolRegistry) {
/// let agent = codetether_agent::agent::Agent::new(info, provider, tools, "system".into());
/// assert!(agent.permissions.is_empty());
/// # }
/// ```
pub type Agent = codetether_agent_runtime::Agent<super::runtime_host::Native>;
