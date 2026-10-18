//! Application-local session binding for this crate's contract.
//! The generated implementation delegates to the existing fields and methods.
pub use codetether_session_metadata::SessionSummary;

/// Bind an application-local session to the existing indexed::Header contract.
///
/// The type must expose the session fields and operations used by this adapter.
/// # Usage
/// ```text
/// codetether_session_listing::impl_session_header!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_header {
    ($session:ty) => {
        impl $crate::indexed::Header for $session {
            fn into_summary(self, count: usize) -> $crate::session_binding::SessionSummary {
                let directory = self.metadata.directory;
                $crate::session_binding::SessionSummary {
                    id: self.id,
                    title: self.title,
                    created_at: self.created_at,
                    updated_at: self.updated_at,
                    message_count: count,
                    agent: self.agent,
                    directory,
                }
            }
        }
    };
}
