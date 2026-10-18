use super::{Control, SessionHandle};
impl<Q, C: Control> SessionHandle<Q, C> {
    /// Reserve a session as the active target before runtime handoff.
    pub fn activate_steering(&self, session_id: &str) -> bool {
        self.active_cancel.prepare(session_id)
    }

    /// Stop accepting steering after a failed handoff.
    pub fn clear_steering(&self) {
        self.active_cancel.clear();
    }

    /// Append input to the currently active prompt run.
    pub fn steer_current(&self, input: C::Input) -> bool {
        self.active_cancel.steer(input)
    }
}
