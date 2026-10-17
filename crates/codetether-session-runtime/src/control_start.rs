//! Single-executor claim of a reserved or idle session.

use crate::{ControlPlane, Execution, control_handoff};

impl ControlPlane {
    /// Claim a session for execution, creating a reservation if it is idle.
    ///
    /// Returns `None` if another executor already claimed it. A reservation
    /// made by a frontend retains its generation and pending cancellation.
    /// # Panics
    /// Panics if the control mutex is poisoned or all run generations are used.
    pub fn begin(&self, session_id: &str) -> Option<Execution> {
        let mut state = self.state();
        if !state.entries.contains_key(session_id) {
            control_handoff::reserve(&mut state, session_id)?;
        }
        let entry = state.entries.get_mut(session_id)?;
        if entry.executing {
            return None;
        }
        entry.executing = true;
        Some(Execution {
            lease: entry.lease.clone(),
            stop: entry.stop.subscribe(),
        })
    }
}
