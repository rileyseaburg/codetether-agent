//! Reservation and executor attachment for one active turn.

use std::sync::Arc;
use tokio::sync::Notify;

/// Identity and cancellation state of one in-flight session turn.
///
/// Callers serialize access when sharing a turn between tasks. No new
/// reservation or attachment is accepted until the current turn is cleared.
///
/// # Examples
/// ```
/// use codetether_session_runtime::ActiveTurn;
/// let mut turn = ActiveTurn::default();
/// assert!(turn.prepare("first"));
/// assert!(!turn.prepare("second"));
/// assert_eq!(turn.clear().as_deref(), Some("first"));
/// ```
#[derive(Default)]
pub struct ActiveTurn {
    session_id: Option<String>,
    pub(crate) cancel: Option<Arc<Notify>>,
    pub(crate) cancel_pending: bool,
}

impl ActiveTurn {
    /// Reserve an idle state for `session_id` before command handoff.
    pub fn prepare(&mut self, session_id: &str) -> bool {
        if self.session_id.is_some() {
            return false;
        }
        self.session_id = Some(session_id.to_string());
        true
    }

    /// Attach an executor to its reservation or an idle turn; deliver pending cancellation.
    pub fn attach(&mut self, session_id: &str, cancel: Arc<Notify>) -> bool {
        match (&self.session_id, &self.cancel) {
            (Some(active), None) if active == session_id => self.cancel = Some(cancel),
            (None, None) => {
                self.session_id = Some(session_id.to_string());
                self.cancel = Some(cancel);
            }
            (Some(_), None | Some(_)) | (None, Some(_)) => return false,
        }
        self.deliver_pending_cancel();
        true
    }

    /// Reset the state and return the session whose inbox needs cleanup.
    pub fn clear(&mut self) -> Option<String> {
        self.cancel_pending = false;
        self.cancel = None;
        self.session_id.take()
    }

    /// Clone the active executor's cancellation notifier, if attached.
    pub fn cancel(&self) -> Option<Arc<Notify>> {
        self.cancel.clone()
    }

    /// Borrow the active session identifier, if reserved or running.
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }
}
