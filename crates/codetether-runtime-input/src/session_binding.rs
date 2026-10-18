//! Application-local session binding for this crate's contract.
//! The generated implementation delegates to the existing fields and methods.
pub use codetether_provider_core::Message;
pub use codetether_session_metadata::SessionMetadata;

/// Bind an application-local session to the existing Source contract.
///
/// The type must expose the session fields and operations used by this adapter.
/// # Usage
/// ```text
/// codetether_runtime_input::impl_session_source!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_source {
    ($session:ty) => {
        impl $crate::Source for $session {
            fn id(&self) -> &str {
                &self.id
            }
            fn agent(&self) -> &str {
                &self.agent
            }
            fn metadata(&self) -> &$crate::session_binding::SessionMetadata {
                &self.metadata
            }
            fn messages(&self) -> &[$crate::session_binding::Message] {
                &self.messages
            }
        }
    };
}
