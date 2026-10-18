//! Application-local session operations for this crate's input contract.
pub use codetether_provider_core::Message;

/// Bind existing session methods to the Source interface.
///
/// Callback paths retain the application's existing publication or access policy.
/// # Usage
/// ```text
/// codetether_recall_context::impl_session_source!(Session, prior_context_allowed);
/// ```
#[macro_export]
macro_rules! impl_session_source {
    ($session:ty, $allow:path) => {
        impl $crate::Source for $session {
            fn prior_context_allowed(&self) -> bool {
                $allow(self)
            }
            fn workspace(&self) -> ::std::option::Option<&::std::path::Path> {
                self.metadata.directory.as_deref()
            }
            fn messages(&self) -> &[$crate::session_binding::Message] {
                &self.messages
            }
            fn id(&self) -> &str {
                &self.id
            }
        }
    };
}
