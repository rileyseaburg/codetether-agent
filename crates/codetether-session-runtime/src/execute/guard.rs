//! Cancellation-safe clearing of one TUI prompt's active ownership.

use crate::handle::Control;

/// Clears active ownership explicitly or when execution unwinds.
/// # Examples
/// ```no_run
/// # fn finish<C: codetether_session_runtime::handle::Control>(active: C) {
/// let guard = codetether_session_runtime::execute::CompletionGuard::new(active);
/// guard.clear();
/// # }
/// ```
pub struct Guard<C: Control>(C);

impl<C: Control> Guard<C> {
    /// Retain the controller until explicit completion or drop.
    pub fn new(active: C) -> Self {
        Self(active)
    }

    /// Clear ownership once and suppress the drop-time fallback.
    pub fn clear(self) {
        self.0.clear();
        std::mem::forget(self);
    }
}

impl<C: Control> Drop for Guard<C> {
    fn drop(&mut self) {
        self.0.clear();
    }
}
