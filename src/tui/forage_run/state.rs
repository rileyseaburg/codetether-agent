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
    /// Final summary text (forage finished successfully).
    Complete(String),
    /// Scan finished and found work; offer to start it. Carries the summary
    /// text, how many were found, and the params needed to re-launch.
    Offer {
        /// Human-readable summary of the scan.
        text: String,
        /// Number of opportunities found.
        selected: usize,
        /// `--top` value to reuse when starting.
        top: usize,
        /// Model to reuse when starting.
        model: Option<String>,
    },
    /// Forage run failed with an error message.
    Error(String),
}

/// A pending "want me to start?" offer awaiting a Y/N answer.
#[derive(Debug, Clone)]
pub struct ForageOffer {
    /// `--top` value to reuse when the user accepts.
    pub top: usize,
    /// Model to reuse when the user accepts.
    pub model: Option<String>,
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
    /// A scan offer awaiting the user's Y/N answer, if any.
    pub pending_offer: Option<ForageOffer>,
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
