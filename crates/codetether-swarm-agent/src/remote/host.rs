//! Application choices used by the shared remote worker adapter.
/// Provider loading and session instructions for a remote sub-agent.
/// # Implementors
/// Applications reuse their agent policy, workspace factory and provider loader.
pub trait RemoteHost: crate::job::JobHost {
    /// Provider loader and lookup retaining the application's registry ownership.
    type Providers: codetether_swarm_orchestrator::Host;
    /// Append the application's session instructions to a prepared system prompt.
    fn append_session_instructions(prompt: String) -> String;
}
