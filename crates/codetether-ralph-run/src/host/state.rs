//! Borrowed state and completion-reporting access.
use crate::*;
/// Access live state without introducing a second run-state owner.
/// # Implementors
/// Return references to the native runtime's current fields.
pub trait State: Send + Sync {
    /// Borrow the current PRD and progress.
    fn state(&self) -> &RalphState;
    /// Update run status after successful scheduling.
    fn state_mut(&mut self) -> &mut RalphState;
    /// Borrow the run configuration.
    fn config(&self) -> &RalphConfig;
    /// Borrow optional persistent storage.
    fn store(&self) -> Option<&Arc<dyn RalphStateStore>>;
    /// Borrow the persistent run identity.
    fn run_id(&self) -> &str;
    /// Send the original lifecycle notification without blocking.
    fn try_send_event(&self, event: RalphEvent);
}
