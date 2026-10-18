//! Application-local session binding for this crate's contract.
//! The generated implementation delegates to the existing fields and methods.
pub use codetether_provider_core::Message;

/// Bind an application-local session to the existing Session contract.
///
/// The type must expose the session fields and operations used by this adapter.
/// # Usage
/// ```text
/// codetether_session_orchestration::impl_session_transcript!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_transcript {
    ($session:ty) => {
        impl $crate::Session for $session {
            fn add_message(&mut self, message: $crate::session_binding::Message) {
                <$session>::add_message(self, message);
            }
            fn messages(&self) -> &[$crate::session_binding::Message] {
                &self.messages
            }
        }
    };
}
