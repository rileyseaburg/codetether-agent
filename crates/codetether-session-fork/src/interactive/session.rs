//! Additional session fields needed for trailing-turn interactive forks.
use crate::ForkSession;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
use codetether_session_metadata::SessionMetadata;
/// Existing storage operations plus direct history installation for TUI forks.
/// # Implementors
/// Native sessions retain their normal fresh-identity and persistence operations.
pub trait Session: ForkSession {
    /// Return page metadata aligned with the current conversation.
    fn pages(&self) -> &[PageKind];
    /// Install copied message and page histories without adding new messages.
    fn install_history(&mut self, messages: Vec<Message>, pages: Vec<PageKind>);
    /// Access the fresh child's metadata for selective inheritance.
    fn metadata_mut(&mut self) -> &mut SessionMetadata;
    /// Attach the process bus after the child replaces the active session.
    fn attach_global_bus_if_missing(&mut self);
}
