//! Parent-scoped child-result collection and delegated insertion.
//!
//! [SubAgentWatch] deduplicates recent bus envelopes against its in-memory IDs
//! and markers already present in the parent transcript. Implement [Host] for
//! the session and call [SubAgentWatch::inject] at the existing prompt boundary.
mod collect;
mod extract;
mod watch;
use codetether_provider_core::Message;
pub use watch::SubAgentWatch;
/// Session operations needed for child-result delivery.
/// # Implementors
/// The native session supplies its transcript and delegated insertion policy.
pub trait Host {
    /// Parent identity used to filter addressed child messages.
    fn session_id(&self) -> &str;
    /// Existing transcript used to suppress previously inserted notices.
    fn history(&self) -> &[Message];
    /// Insert through the session's delegated-message path.
    fn add_delegated_message(&mut self, message: Message);
}
