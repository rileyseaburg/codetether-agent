//! Retain user cancellation in the gap between reservation and executor startup.

use super::ActiveTurn;

impl ActiveTurn {
    pub(in super::super) fn request_cancel(&mut self) -> bool {
        if self.session_id.is_none() {
            return false;
        }
        self.cancel_pending = true;
        self.deliver_pending_cancel();
        true
    }

    pub(super) fn deliver_pending_cancel(&mut self) {
        if self.cancel_pending
            && let Some(cancel) = &self.cancel
        {
            cancel.notify_one();
            self.cancel_pending = false;
        }
    }
}

#[cfg(test)]
#[path = "active_turn_cancel_tests.rs"]
mod tests;
