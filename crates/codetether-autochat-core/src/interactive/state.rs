//! Receiver and running state for an interactive autochat relay.

use tokio::sync::mpsc::UnboundedReceiver;

use super::events::AutochatUiEvent;

/// State for a relay whose session-event payload is `E`.
///
/// # Examples
/// ```
/// let state = codetether_autochat_core::interactive::state::AutochatState::<()>::default();
/// assert!(!state.running);
/// assert!(state.rx.is_none());
/// ```
pub struct AutochatState<E> {
    /// Whether the autochat relay is currently running.
    pub running: bool,
    /// Channel receiving events from the relay worker.
    pub rx: Option<UnboundedReceiver<AutochatUiEvent<E>>>,
}

impl<E> Default for AutochatState<E> {
    fn default() -> Self {
        Self {
            running: false,
            rx: None,
        }
    }
}
