//! Cancellation-safe clearing of one TUI prompt's active ownership.

use crate::handle::Control;

pub(super) struct Guard<C: Control>(C);

impl<C: Control> Guard<C> {
    pub(super) fn new(active: C) -> Self {
        Self(active)
    }

    pub(super) fn clear(self) {
        self.0.clear();
        std::mem::forget(self);
    }
}

impl<C: Control> Drop for Guard<C> {
    fn drop(&mut self) {
        self.0.clear();
    }
}
