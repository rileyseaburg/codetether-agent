//! Native session construction and ordered history mutation.
use super::SessionView;
use codetether_agent_core::ToolUse;
use codetether_provider_core::Message;
/// Construct and update native sessions using their existing methods.
/// # Implementors
/// Keep human-message semantics distinct from ordinary message append.
#[async_trait::async_trait]
pub trait SessionWrite: SessionView {
    /// Create the session for a new swarm task.
    /// # Errors
    /// Returns native session initialization failures.
    async fn new_session() -> anyhow::Result<Self::Session>;
    /// Save pending messages before a tool edits the same session through storage.
    /// # Errors
    /// Propagates persistence errors before the tool is allowed to execute.
    async fn persist_session(session: &mut Self::Session) -> anyhow::Result<()>;
    /// Reload externally edited context without discarding runtime services.
    /// # Errors
    /// Propagates refresh errors so replay cannot continue with stale context.
    async fn refresh_session(session: &mut Self::Session) -> anyhow::Result<()>;
    /// Append a human turn.
    fn add_human(session: &mut Self::Session, message: Message);
    /// Append an assistant or tool message.
    fn add_message(session: &mut Self::Session, message: Message);
    /// Append a tool-use record.
    fn record_tool(session: &mut Self::Session, usage: ToolUse);
}
