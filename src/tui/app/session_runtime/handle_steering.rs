//! Active-turn steering methods on the TUI runtime handle.

use crate::session::helper::steering::SteeringInput;

use super::TuiSessionHandle;

impl TuiSessionHandle {
    /// Reserve a session as the active target before runtime handoff.
    pub(crate) fn activate_steering(&self, session_id: &str) -> bool {
        self.active_cancel.prepare(session_id)
    }

    /// Stop accepting steering after a failed handoff.
    pub(crate) fn clear_steering(&self) {
        self.active_cancel.clear();
    }

    /// Append input to the currently active prompt run.
    pub(crate) fn steer_current(&self, input: SteeringInput) -> bool {
        self.active_cancel.steer(input)
    }
}
