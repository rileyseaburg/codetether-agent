//! Mutable identity and cancellation state for one active TUI turn.

use std::sync::Arc;

use tokio::sync::Notify;

/// Identity and cancellation notifier for one in-flight session turn.
#[derive(Default)]
pub(super) struct ActiveTurn {
    session_id: Option<String>,
    cancel: Option<Arc<Notify>>,
}

impl ActiveTurn {
    /// Reserve an idle state for `session_id` before command handoff.
    pub(super) fn prepare(&mut self, session_id: &str) -> bool {
        if self.session_id.is_some() {
            return false;
        }
        self.session_id = Some(session_id.to_string());
        true
    }

    /// Attach the runtime notifier to a prepared or idle state.
    pub(super) fn attach(&mut self, session_id: &str, cancel: Arc<Notify>) -> bool {
        match (&self.session_id, &self.cancel) {
            (Some(active), None) if active == session_id => self.cancel = Some(cancel),
            (None, None) => {
                self.session_id = Some(session_id.to_string());
                self.cancel = Some(cancel);
            }
            (Some(_), None | Some(_)) | (None, Some(_)) => return false,
        }
        true
    }

    /// Reset the state and return the session whose inbox needs cleanup.
    pub(super) fn clear(&mut self) -> Option<String> {
        self.cancel = None;
        self.session_id.take()
    }

    /// Clone the active cancellation notifier, when execution has started.
    pub(super) fn cancel(&self) -> Option<Arc<Notify>> {
        self.cancel.clone()
    }

    /// Borrow the active session identifier across synchronized inbox access.
    pub(super) fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }
}
