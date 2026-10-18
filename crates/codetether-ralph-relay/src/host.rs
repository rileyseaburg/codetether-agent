//! Native registry and session operations used by relay orchestration.
use crate::*;
/// Construct and prompt native sessions without moving their implementation here.
/// # Implementors
/// Bind the existing provider registry and session methods used by Ralph.
#[async_trait::async_trait]
pub trait Host: Send + Sync + 'static {
    /// Provider registry shared by every relay turn.
    type Registry: Send + Sync + 'static;
    /// Owned session restored to the team after a successful task join.
    type Session: Send + 'static;
    /// Enumerate provider names in the original registry order.
    fn providers(registry: &Self::Registry) -> Vec<&str>;
    /// Construct and initialize a session after relay provider selection.
    /// # Errors
    /// Returns native session initialization errors.
    async fn new_session(
        model: String,
        name: &str,
        instructions: &str,
        bus: &Arc<AgentBus>,
    ) -> anyhow::Result<Self::Session>;
    /// Prompt an owned session and return it alongside the full native result.
    /// The caller restores the session before inspecting a prompt error.
    async fn prompt(
        session: Self::Session,
        baton: String,
        events: mpsc::Sender<SessionEvent<Self::Session>>,
        registry: Arc<Self::Registry>,
    ) -> (Self::Session, anyhow::Result<SessionResult>);
}
