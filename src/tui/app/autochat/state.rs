//! Autochat state fields for AppState.

use tokio::sync::mpsc::UnboundedReceiver;

use super::events::AutochatUiEvent;

/// Autochat state attached to AppState.
pub struct AutochatState {
    /// Whether the autochat relay is currently running.
    pub running: bool,
    /// Channel receiving events from the relay worker.
    pub rx: Option<UnboundedReceiver<AutochatUiEvent>>,
}

impl Default for AutochatState {
    fn default() -> Self {
        Self {
            running: false,
            rx: None,
        }
    }
}
