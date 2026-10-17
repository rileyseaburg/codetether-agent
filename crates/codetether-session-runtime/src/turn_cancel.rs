//! Cancellation retained until an executor can receive it.

use crate::ActiveTurn;

impl ActiveTurn {
    /// Request cancellation, retaining the signal during executor handoff.
    ///
    /// Returns `false` when idle. Repeated requests before attachment coalesce
    /// into one notification, as do pending permits on [`tokio::sync::Notify`].
    pub fn request_cancel(&mut self) -> bool {
        if self.session_id().is_none() {
            return false;
        }
        self.cancel_pending = true;
        self.deliver_pending_cancel();
        true
    }

    pub(crate) fn deliver_pending_cancel(&mut self) {
        if !self.cancel_pending {
            return;
        }
        if let Some(cancel) = &self.cancel {
            cancel.notify_one();
            self.cancel_pending = false;
        }
    }
}
