//! Read-only ownership checks for frontend adapters.

use crate::{ControlPlane, Lease};

impl ControlPlane {
    /// Check cancellation before dispatching another effectful operation.
    ///
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn is_cancelled(&self, session_id: &str) -> bool {
        self.state()
            .entries
            .get(session_id)
            .is_some_and(|entry| *entry.stop.borrow())
    }

    /// Return whether this exact run still owns its session.
    ///
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn is_current(&self, lease: &Lease) -> bool {
        self.state()
            .entries
            .get(lease.session_id())
            .is_some_and(|entry| entry.lease == *lease)
    }
}
