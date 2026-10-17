//! Serialize application cleanup with ownership replacement.

use crate::{ControlPlane, Lease};

impl ControlPlane {
    /// Run a synchronous adapter action only while this exact lease is current.
    ///
    /// The callback must not re-enter this control plane or wait on async work.
    /// # Panics
    /// Panics if the mutex is poisoned or the callback panics.
    pub fn with_current<T>(&self, lease: &Lease, action: impl FnOnce() -> T) -> Option<T> {
        let state = self.state();
        state
            .entries
            .get(lease.session_id())
            .filter(|entry| entry.lease == *lease)
            .map(|_| action())
    }

    /// Clean adapter resources before allowing a replacement reservation.
    ///
    /// The callback must not re-enter this control plane or wait on async work.
    /// # Panics
    /// Panics if the mutex is poisoned or the callback panics.
    pub fn finish_with(&self, lease: &Lease, cleanup: impl FnOnce()) -> bool {
        let mut state = self.state();
        if state
            .entries
            .get(lease.session_id())
            .is_none_or(|entry| entry.lease != *lease)
        {
            return false;
        }
        cleanup();
        state.entries.remove(lease.session_id());
        true
    }

    /// Clean and release an unstarted reservation, preserving attached executors.
    ///
    /// The callback must not re-enter this control plane or wait on async work.
    /// # Panics
    /// Panics if the mutex is poisoned or the callback panics.
    pub fn release_prepared_with(&self, lease: &Lease, cleanup: impl FnOnce()) -> bool {
        let mut state = self.state();
        if !state.entries.get(lease.session_id()).is_some_and(|entry| {
            entry.lease == *lease && !entry.executing && entry.turn.cancel().is_none()
        }) {
            return false;
        }
        cleanup();
        state.entries.remove(lease.session_id());
        true
    }
}
