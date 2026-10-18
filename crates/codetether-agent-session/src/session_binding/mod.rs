//! Application-local settings and lifecycle implementations for managed sessions.
mod access;
mod runtime;
pub use anyhow;
pub use async_trait::async_trait;
pub use chrono::Utc;
pub use codetether_bus_core::global;
pub use codetether_context_policy::summary_index::SummaryIndex;
pub use codetether_provider_core::Message;

/// Bind managed-agent session operations to existing fields and methods.
///
/// The prior-context callback keeps the application's current access policy.
/// # Usage
/// ```text
/// codetether_agent_session::impl_managed_session!(Session, prior_context_allowed);
/// ```
#[macro_export]
macro_rules! impl_managed_session {
    ($session:ty, $prior:path) => {
        $crate::__managed_session_access!($session, $prior);
        $crate::__managed_session_runtime!($session, $prior);
    };
}
