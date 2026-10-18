//! Native execution and UI operations required by swarm commands.
use crate::{SwarmEvent, SwarmViewState};
use codetether_session_control::ControlHandle;
use codetether_swarm_core::{DecompositionStrategy, SwarmConfig};
use std::future::Future;
use tokio::sync::mpsc::Sender;

/// Supplies executor construction and application bus attachment.
/// Implement this on a stateless application adapter.
pub trait Host {
    /// Execute the task with the supplied settings, events and shared control.
    ///
    /// # Errors
    /// Returns the execution error text for publication as a swarm error event.
    fn execute(
        config: SwarmConfig,
        control: ControlHandle,
        events: Sender<SwarmEvent>,
        task: String,
        strategy: DecompositionStrategy,
    ) -> impl Future<Output = Result<(), String>> + Send;
}

/// Application presentation operations used by swarm commands.
/// Implement this for the application's chat view.
pub trait View {
    /// Borrow the monitor state and shared control handle.
    fn swarm(&mut self) -> &mut SwarmViewState;
    /// Switch to the monitor view.
    fn show_monitor(&mut self);
    /// Replace the status line.
    fn set_status(&mut self, status: String);
    /// Append a system message.
    fn push_system(&mut self, text: String);
    /// Scroll chat to the latest message.
    fn scroll_to_bottom(&mut self);
}
