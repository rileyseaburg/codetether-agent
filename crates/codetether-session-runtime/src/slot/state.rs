use super::ViewSource;

/// Holds the canonical session unless a runtime prompt owns it.
/// # Examples
/// ```
/// use codetether_session_runtime::slot::{SessionSlot, ViewSource};
/// struct Session(String);
/// impl ViewSource for Session {
///     type View = String;
///     fn snapshot(&self) -> String { self.0.clone() }
/// }
/// let mut slot = SessionSlot::new(Session("ready".into()));
/// let session = slot.take_for_prompt().unwrap();
/// assert!(slot.borrow().is_none());
/// assert_eq!(slot.view(), "ready");
/// slot.restore(session);
/// assert!(slot.borrow().is_some());
/// ```
pub struct SessionSlot<S: ViewSource> {
    session: Option<S>,
    view: S::View,
}

impl<S: ViewSource> SessionSlot<S> {
    /// Create a slot around the initial active session.
    pub fn new(session: S) -> Self {
        let view = session.snapshot();
        Self {
            session: Some(session),
            view,
        }
    }

    /// Borrow the active session when it is not checked out.
    pub fn borrow(&self) -> Option<&S> {
        self.session.as_ref()
    }

    /// Mutably borrow the active session when it is not checked out.
    pub fn borrow_mut(&mut self) -> Option<&mut S> {
        self.session.as_mut()
    }

    /// View data available even while the session is checked out.
    pub fn view(&self) -> &S::View {
        &self.view
    }

    /// Refresh the retained view only when the session is currently owned.
    pub fn refresh_view(&mut self) {
        if let Some(session) = self.session.as_ref() {
            self.view = session.snapshot();
        }
    }

    /// Move the active session into a prompt request.
    pub fn take_for_prompt(&mut self) -> Option<S> {
        let session = self.session.take()?;
        self.view = session.snapshot();
        Some(session)
    }

    /// Restore a session returned by the runtime.
    pub fn restore(&mut self, session: S) {
        self.view = session.snapshot();
        self.session = Some(session);
    }
}
