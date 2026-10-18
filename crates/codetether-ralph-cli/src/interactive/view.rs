//! Interactive client state needed for Ralph status and launch updates.
use codetether_ralph_core::events::{RalphEvent, RalphStoryInfo};
use tokio::sync::mpsc;
/// Presentation operations used by Ralph commands.
/// # Implementors
/// Interactive clients bind these operations to their existing view state.
pub trait View {
    /// Replace the current status message.
    fn set_status(&mut self, status: String);
    /// Append a system message using the client's existing scrolling behavior.
    fn push_system_message(&mut self, message: String);
    /// Attach the background loop event stream to the monitor.
    fn attach_event_rx(&mut self, events: mpsc::Receiver<RalphEvent>);
    /// Select the Ralph monitor view.
    fn show_monitor(&mut self);
    /// Return the currently displayed story records.
    fn stories(&self) -> &[RalphStoryInfo];
    /// Return the monitor's current iteration count.
    fn current_iteration(&self) -> usize;
    /// Return the monitor's iteration limit.
    fn max_iterations(&self) -> usize;
}
