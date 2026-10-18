use codetether_session_events::SessionEvent;
use std::future::Future;
use tokio::sync::mpsc;
/// Native persistence and final-answer services.
/// # Implementors
/// The runner supplies session storage, archival and evidence gating.
pub trait Host: Send {
    /// Snapshot carried by live events.
    type Snapshot: Send;
    /// Save session history before reporting completion.
    /// # Errors
    /// Propagates session persistence failures.
    fn save(&mut self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Archive the session event stream using the existing best-effort policy.
    fn archive(&mut self) -> impl Future<Output = ()> + Send;
    /// Optional completion-event sender.
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent<Self::Snapshot>>>;
    /// Apply evidence checks to the trimmed final output.
    fn final_text(&self) -> String;
    /// Read the identifier for the completed session.
    fn session_id(&self) -> &str;
}
