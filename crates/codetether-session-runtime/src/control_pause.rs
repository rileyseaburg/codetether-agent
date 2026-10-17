//! Shared pause state for orchestration and model-loop adapters.

use crate::{ControlPlane, Lease};

impl ControlPlane {
    /// Pause this run without changing a durable goal or granting permission.
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn pause(&self, lease: &Lease) -> bool {
        self.set_paused(lease, true)
    }

    /// Release a transient pause; durable goal gates remain authoritative.
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn resume(&self, lease: &Lease) -> bool {
        self.set_paused(lease, false)
    }

    /// Read the transient pause flag for this exact run.
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn is_paused(&self, lease: &Lease) -> bool {
        self.state()
            .entries
            .get(lease.session_id())
            .is_some_and(|entry| entry.lease == *lease && entry.paused)
    }

    fn set_paused(&self, lease: &Lease, paused: bool) -> bool {
        let mut state = self.state();
        let Some(entry) = state
            .entries
            .get_mut(lease.session_id())
            .filter(|entry| entry.lease == *lease)
        else {
            return false;
        };
        entry.paused = paused;
        true
    }
}
