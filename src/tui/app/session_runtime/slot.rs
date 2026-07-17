//! Optional owner for the active TUI session.

use crate::session::Session;

use super::SessionView;

/// Holds the canonical session unless a runtime prompt owns it.
pub(crate) struct SessionSlot {
    session: Option<Session>,
    view: SessionView,
}

impl SessionSlot {
    /// Create a slot around the initial active session.
    pub(crate) fn new(session: Session) -> Self {
        let view = SessionView::from_session(&session);
        Self {
            session: Some(session),
            view,
        }
    }

    /// Borrow the active session when it is not checked out.
    pub(crate) fn borrow(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    /// Mutably borrow the active session when it is not checked out.
    pub(crate) fn borrow_mut(&mut self) -> Option<&mut Session> {
        self.session.as_mut()
    }

    /// View data available even while the session is checked out.
    pub(crate) fn view(&self) -> &SessionView {
        &self.view
    }

    pub(crate) fn refresh_view(&mut self) {
        if let Some(session) = self.session.as_ref() {
            self.view = SessionView::from_session(session);
        }
    }

    /// Move the active session into a prompt request.
    pub(crate) fn take_for_prompt(&mut self) -> Option<Session> {
        let session = self.session.take()?;
        self.view = SessionView::from_session(&session);
        Some(session)
    }

    /// Restore a session returned by the runtime.
    pub(crate) fn restore(&mut self, session: Session) {
        self.view = SessionView::from_session(&session);
        self.session = Some(session);
    }
}
