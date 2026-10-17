//! Subscribers observe a run without claiming a second executor.

use crate::{ControlPlane, Execution, Lease};

impl ControlPlane {
    /// Observe cancellation for this generation without taking execution ownership.
    ///
    /// Returns `None` for stale or released leases. Subscribers resolve when the
    /// owner cancels or releases the run, including a previously queued cancel.
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn subscribe(&self, lease: &Lease) -> Option<Execution> {
        self.state()
            .entries
            .get(lease.session_id())
            .filter(|entry| entry.lease == *lease)
            .map(|entry| Execution {
                lease: lease.clone(),
                stop: entry.stop.subscribe(),
            })
    }

    /// Report cancellation or loss of this exact ownership generation.
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn is_stopped(&self, lease: &Lease) -> bool {
        self.state()
            .entries
            .get(lease.session_id())
            .is_none_or(|entry| entry.lease != *lease || *entry.stop.borrow())
    }
}
