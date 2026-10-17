//! Read-only session statistics needed for the budget report.
use codetether_context_policy::pages::PageKind;
/// Existing session storage plus the budget report's three read-only inputs.
///
/// # Implementors
/// Implement for the application's session type without copying transcript data.
pub trait BudgetSource: codetether_context_loader::Source {
    /// Working-window size used by the session store.
    const WINDOW: usize;
    /// Number of messages in the loaded window.
    fn messages_len(&self) -> usize;
    /// Number of cached summaries.
    fn cached_summaries(&self) -> usize;
    /// Page classifications from the loaded session.
    fn pages(&self) -> &[PageKind];
}
