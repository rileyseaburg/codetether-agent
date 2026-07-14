//! Thread-local scope for panics that the renderer intends to catch.

use std::cell::Cell;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
}

/// Resets the recoverable-render marker when the render attempt ends.
pub(in crate::tui::app) struct Guard;

/// Mark the current thread's render panic as locally recoverable.
pub(in crate::tui::app) fn enter() -> Guard {
    ACTIVE.set(true);
    Guard
}

/// Return whether the current thread is inside a recoverable render attempt.
pub(in crate::tui::app) fn active() -> bool {
    ACTIVE.get()
}

impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.set(false);
    }
}

#[cfg(test)]
#[path = "recovery_scope_tests.rs"]
mod tests;
