//! Background state for the TUI `/forage` command.
//!
//! Holds a channel that streams human-readable progress and result
//! messages from a background forage run back into the chat view.

use tokio::sync::mpsc;

/// A single update emitted by a background forage run.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::forage_run::ForageUpdate;
///
/// let msg = ForageUpdate::Status("scanning OKRs".to_string());
/// assert!(matches!(msg, ForageUpdate::Status(_)));
/// ```
#[derive(Debug, Clone)]
pub enum ForageUpdate {
    /// Informational progress line.
    Status(String),
    /// Final summary text for a scan-only run.
    ScanComplete(String),
    /// Final summary text for a successful execution run.
    Complete(String),
    /// Forage run failed with an error message.
    Error(String),
}

/// Receiver-side state for a running forage task.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::forage_run::ForageState;
///
/// let state = ForageState::new();
/// assert!(!state.active);
/// ```
#[derive(Default)]
pub struct ForageState {
    /// Whether a forage run is currently attached.
    pub active: bool,
    /// Pending updates from the background run.
    pub rx: Option<mpsc::Receiver<ForageUpdate>>,
}

impl ForageState {
    /// Create an idle forage state with no attached run.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach a receiver for a freshly launched forage run.
    pub fn attach_rx(&mut self, rx: mpsc::Receiver<ForageUpdate>) {
        self.rx = Some(rx);
        self.active = true;
    }
}
