//! Application execution and presentation boundaries for interactive forage.
use super::ForageState;
use codetether_cli_options::ForageArgs;
use codetether_forage_core::ForageRunSummary;
use std::future::Future;

/// Supplies the application forage engine used by [super::spawn_forage_run].
/// Implement this on a stateless application adapter.
pub trait Host {
    /// Set console suppression before launching the background task.
    fn set_quiet(quiet: bool);
    /// Execute one configured forage run.
    ///
    /// # Errors
    /// Returns the underlying forage execution failure.
    fn execute(args: ForageArgs) -> impl Future<Output = anyhow::Result<ForageRunSummary>> + Send;
}

/// Presents launch state; implement this on the application's chat view.
pub trait View {
    /// Borrow the receiver-side run state.
    fn state(&mut self) -> &mut ForageState;
    /// Replace the status line.
    fn set_status(&mut self, status: String);
    /// Append a system message.
    fn push_system(&mut self, text: String);
    /// Scroll the chat to its latest message.
    fn scroll_to_bottom(&mut self);
}
