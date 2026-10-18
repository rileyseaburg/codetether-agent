//! Native session restore, bus and identity adapters.
use super::Native;
use crate::session::Session;
impl codetether_session_events::lifecycle::Session for Session {
    fn attach_bus(&mut self) {
        self.attach_global_bus_if_missing();
    }
    fn session_id(&self) -> String {
        self.id.clone()
    }
}
impl codetether_session_events::lifecycle::Sync for Native<'_> {
    type Session = Session;
    fn restore(&mut self, session: Session) {
        self.slot.restore(session);
    }
    fn current_session(&mut self) -> Option<&mut Session> {
        self.slot.borrow_mut()
    }
    fn set_session_id(&mut self, id: String) {
        self.app.state.session_id = Some(id);
    }
}
