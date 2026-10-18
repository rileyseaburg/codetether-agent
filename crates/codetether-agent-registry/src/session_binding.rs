//! Application-local identity binding for the live-agent registry.
/// Bind the registry session identity to an existing session ID field.
/// # Usage
/// ```text
/// codetether_agent_registry::impl_session_identity!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_identity {
    ($session:ty) => {
        impl $crate::Session for $session {
            fn id(&self) -> &str {
                &self.id
            }
        }
    };
}
