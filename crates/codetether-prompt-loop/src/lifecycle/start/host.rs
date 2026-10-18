use codetether_session_events::SessionEvent;
use std::future::Future;
use tokio::sync::mpsc;
/// Native services required to begin a prompt step.
/// # Implementors
/// The runner supplies execution control, steering, model setup and budgets.
pub trait Host: Send {
    /// Snapshot type in live events.
    type Snapshot: Send;
    /// Check the session execution boundary.
    /// # Errors
    /// Propagates cancellation or execution-control failures.
    fn ensure_running(&mut self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Insert pending user steering into session history.
    fn drain_steering(&mut self);
    /// Restore and prepare the step's model and tools.
    /// # Errors
    /// Propagates native model-preparation failures.
    fn prepare(&mut self) -> anyhow::Result<()>;
    /// Insert unseen child-agent notices.
    fn inject_notices(&mut self);
    /// Optional thinking-event sender.
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent<Self::Snapshot>>>;
    /// Enforce the configured completion cost budget.
    /// # Errors
    /// Returns an error when the budget prevents execution.
    fn enforce_cost(&self) -> anyhow::Result<()>;
}
