//! Generation-checked cancellation and cleanup across frontend adapters.

use crate::{ControlPlane, Lease};

impl ControlPlane {
    /// Signal the executor and frontend only if this lease still owns the session.
    ///
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn cancel(&self, lease: &Lease) -> bool {
        let mut state = self.state();
        let Some(entry) = state
            .entries
            .get_mut(lease.session_id())
            .filter(|entry| entry.lease == *lease)
        else {
            return false;
        };
        entry.stop.send_replace(true);
        entry.turn.request_cancel();
        true
    }

    /// Cancel the current owner by session ID at a trusted transport boundary.
    ///
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn cancel_session(&self, session_id: &str) -> bool {
        let lease = self
            .state()
            .entries
            .get(session_id)
            .map(|entry| entry.lease.clone());
        lease.is_some_and(|lease| self.cancel(&lease))
    }

    /// Release this exact run, leaving a replacement owner untouched.
    ///
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn finish(&self, lease: &Lease) -> bool {
        self.finish_with(lease, || {})
    }

    /// Release an unstarted, unattached reservation after a rejected handoff.
    ///
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn release_prepared(&self, lease: &Lease) -> bool {
        self.release_prepared_with(lease, || {})
    }
}
