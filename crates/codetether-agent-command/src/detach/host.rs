//! Application operations required for detached background agents.
use codetether_agent_tree::SpawnedAgent;
/// Agent registry and presentation access for the detach workflow.
/// # Implementors
/// Frontends supply their native session type and existing UI operations.
pub trait Host {
    /// Session storage with detached snapshot field bindings.
    type Session: codetether_session_fork::detached::Session;
    /// Whether the current conversation is processing a response.
    fn is_processing(&self) -> bool;
    /// Whether an agent name is already registered.
    fn contains_name(&self, name: &str) -> bool;
    /// Resolve the model shown for a newly registered agent.
    fn current_model_id(&self) -> Option<String>;
    /// Insert the new background agent.
    fn register(&mut self, name: String, agent: SpawnedAgent<Self::Session>);
    /// Resolve the configured history window when constructing the snapshot.
    fn resume_window(&self) -> usize;
    /// Replace visible status text.
    fn set_status(&mut self, status: String);
    /// Switch to the background-agent view.
    fn show_subagents(&mut self);
    /// Append a system message to the transcript.
    fn push_system(&mut self, content: String);
}
