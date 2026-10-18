//! Application-local session operations for this crate's input contract.
pub use anyhow;
pub use codetether_provider_core::Message;

/// Bind existing session methods to the checkpoint::Host interface.
///
/// Callback paths retain the application's existing publication or access policy.
/// # Usage
/// ```text
/// codetether_provider_step::impl_session_checkpoint!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_checkpoint {
    ($session:ty) => {
        impl $crate::checkpoint::Host for $session {
            fn append(&mut self, message: $crate::session_binding::Message) {
                self.add_message(message);
            }
            async fn save(&self) -> $crate::session_binding::anyhow::Result<()> {
                <$session>::save(self).await
            }
        }
    };
}
