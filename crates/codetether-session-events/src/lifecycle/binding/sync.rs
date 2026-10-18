//! Frontend sync operations for session-event lifecycles.
/// Supporting implementation for [crate::declare_event_lifecycle].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_event_sync {
    ($native:ident, $session:ty) => {
        impl $crate::lifecycle::Session for $session {
            fn attach_bus(&mut self) {
                self.attach_global_bus_if_missing();
            }
            fn session_id(&self) -> String {
                self.id.clone()
            }
        }
        impl $crate::lifecycle::Sync for $native<'_> {
            type Session = $session;
            fn restore(&mut self, session: $session) {
                self.slot.restore(session);
            }
            fn current_session(&mut self) -> Option<&mut $session> {
                self.slot.borrow_mut()
            }
            fn set_session_id(&mut self, id: String) {
                self.app.state.session_id = Some(id);
            }
        }
    };
}
