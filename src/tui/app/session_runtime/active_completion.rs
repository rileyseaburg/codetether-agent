//! Cancellation-safe clearing of one TUI prompt's active ownership.

use super::active_cancel::ActiveCancel;

pub(super) struct Guard(ActiveCancel);

impl Guard {
    pub(super) fn new(active: ActiveCancel) -> Self {
        Self(active)
    }

    pub(super) fn clear(self) {
        self.0.clear();
        std::mem::forget(self);
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.0.clear();
    }
}
