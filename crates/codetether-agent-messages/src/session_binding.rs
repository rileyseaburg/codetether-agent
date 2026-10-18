//! Existing session transcript binding for child-result delivery.
pub use codetether_provider_core::Message;
/// Bind child notices to existing history and delegated-message insertion.
/// # Usage
/// ```text
/// codetether_agent_messages::impl_session_notices!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_notices {
    ($session:ty) => {
        impl $crate::notice::Host for $session {
            fn session_id(&self) -> &str {
                &self.id
            }
            fn history(&self) -> &[$crate::session_binding::Message] {
                <$session>::history(self)
            }
            fn add_delegated_message(&mut self, message: $crate::session_binding::Message) {
                <$session>::add_delegated_message(self, message);
            }
        }
    };
}
