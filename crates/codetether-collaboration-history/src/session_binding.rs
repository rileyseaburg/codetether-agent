//! Application-local session binding for this crate's contract.
//! The generated implementation delegates to the existing fields and methods.
pub use anyhow;
pub use async_trait::async_trait;
pub use codetether_provider_core::Message;

/// Bind an application-local session to the existing HistorySource contract.
///
/// The type must expose the session fields and operations used by this adapter.
/// # Usage
/// ```text
/// codetether_collaboration_history::impl_history_source!(Session);
/// ```
#[macro_export]
macro_rules! impl_history_source {
    ($session:ty) => {
        #[$crate::session_binding::async_trait]
        impl $crate::HistorySource for $session {
            async fn load(parent_id: &str) -> $crate::session_binding::anyhow::Result<Self> {
                <$session>::load(parent_id).await
            }
            fn history(&self) -> &[$crate::session_binding::Message] {
                <$session>::history(self)
            }
            fn add_message(&mut self, message: $crate::session_binding::Message) {
                <$session>::add_message(self, message)
            }
        }
    };
}
