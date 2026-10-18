//! Concrete native session slot with a TUI metadata snapshot.
use super::SessionView;
use crate::session::Session;
pub(crate) type SessionSlot = codetether_session_runtime::slot::SessionSlot<Session>;
impl codetether_session_runtime::slot::ViewSource for Session {
    type View = SessionView;
    fn snapshot(&self) -> SessionView {
        SessionView::from_session(self)
    }
}
