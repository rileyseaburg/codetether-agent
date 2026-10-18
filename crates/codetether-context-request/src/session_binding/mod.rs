//! Application-local session binding with explicit context-service callbacks.
mod host;
mod source;
pub use anyhow;
pub use async_trait::async_trait;
pub use codetether_context_policy::{
    derivation::DerivePolicy, projection::helpers::DerivedContext,
};
pub use codetether_provider_core::{Message, Provider};

/// Bind an application's existing context services to a session type.
///
/// Receives the type, effective-policy, preparation, derivation and recall paths.
/// # Usage
/// ```text
/// codetether_context_request::impl_session_context!(
///     Session, effective_policy, prepare, derive_with_policy, prefetch);
/// ```
#[macro_export]
macro_rules! impl_session_context {
    ($session:ty, $effective:path, $prepare:path, $derive:path, $prefetch:path) => {
        $crate::__context_request_session_source!($session, $effective, $prepare);
        $crate::__context_request_session_host!($session, $derive, $prefetch);
    };
}
