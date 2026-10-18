//! Application-local session operations for this crate's input contract.
pub use codetether_provider_core::Message;

/// Bind existing session methods to the Transcript interface.
///
/// Callback paths retain the application's existing publication or access policy.
/// # Usage
/// ```text
/// codetether_session_steering::impl_session_transcript!(Session, publish_user_prompt);
/// ```
#[macro_export]
macro_rules! impl_session_transcript {
    ($session:ty, $publish:path) => {
        impl $crate::Transcript for $session {
            fn id(&self) -> &str {
                &self.id
            }
            fn add_human_message(&mut self, message: $crate::session_binding::Message) {
                <$session>::add_human_message(self, message);
            }
            fn publish_user_prompt(&self, text: &str) {
                $publish(self, text, None);
            }
        }
    };
}
