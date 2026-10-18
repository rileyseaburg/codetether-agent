//! Managed session access using existing application state.
/// Internal managed-session binding implementation.
#[doc(hidden)]
#[macro_export]
macro_rules! __managed_session_access {
    ($session:ty, $prior:path) => {
        impl $crate::SessionAccess for $session {
            fn settings(&mut self) -> $crate::Settings<'_> {
                $crate::Settings {
                    model: &mut self.metadata.model,
                    directory: &mut self.metadata.directory,
                    inherited_prior_context_allowed: &mut self
                        .metadata
                        .inherited_prior_context_allowed,
                    auto_apply_edits: &mut self.metadata.auto_apply_edits,
                    workspace_pinned: self.metadata.workspace_pinned,
                }
            }
            fn messages_mut(
                &mut self,
            ) -> ::std::slice::IterMut<'_, $crate::session_binding::Message> {
                self.messages.iter_mut()
            }
            fn first_message_mut(
                &mut self,
            ) -> ::std::option::Option<&mut $crate::session_binding::Message> {
                self.messages.first_mut()
            }
            fn set_workspace_pinned(&mut self, pinned: bool) {
                self.metadata.workspace_pinned = pinned;
            }
            fn reset_summary_index(&mut self) {
                self.summary_index = $crate::session_binding::SummaryIndex::new();
            }
            fn add_message(&mut self, message: $crate::session_binding::Message) {
                <$session>::add_message(self, message);
            }
        }
    };
}
