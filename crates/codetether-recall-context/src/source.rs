//! Immutable session inputs for automatic recall injection.
use codetether_provider_core::Message;
use std::path::Path;
/// Prior-context authorization and inputs from the current native session.
/// # Implementors
/// Call the application's existing authorization policy in prior_context_allowed.
pub trait Source {
    /// Whether prior-session context may be used for this request.
    fn prior_context_allowed(&self) -> bool;
    /// Workspace used to scope the recall catalog.
    fn workspace(&self) -> Option<&Path>;
    /// Transcript from which the latest user text is selected.
    fn messages(&self) -> &[Message];
    /// Current session ID, excluded from search results.
    fn id(&self) -> &str;
}
