//! Policy transitions precede the application's ordinary transcript append.
/// Bind trusted and delegated message ingress to an application-local session.
///
/// The session must expose its metadata/messages and an existing add_message method.
/// The generated methods retain crate-level visibility.
/// # Usage
/// ```text
/// codetether_prior_context_access::impl_session_ingress!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_ingress {
    ($session:ty) => {
        impl $session {
            /// Record trusted human policy updates before appending the message.
            pub(crate) fn add_human_message(&mut self, message: $crate::session_binding::Message) {
                self.metadata.progress.human_input(&message);
                $crate::trusted(&mut self.metadata, &self.messages, &message);
                self.add_message(message);
            }
            /// Record restricted delegated policy updates before appending the message.
            pub(crate) fn add_delegated_message(
                &mut self,
                message: $crate::session_binding::Message,
            ) {
                $crate::delegated(&mut self.metadata, &self.messages, &message);
                self.add_message(message);
            }
        }
        impl $crate::session_binding::Source for $session {
            fn metadata(&self) -> &$crate::session_binding::SessionMetadata {
                &self.metadata
            }
            fn messages(&self) -> &[$crate::session_binding::Message] {
                &self.messages
            }
        }
    };
}
