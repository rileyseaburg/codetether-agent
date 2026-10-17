//! Borrowed settings and tracked transcript access for child-session rules.
use crate::Settings;
use codetether_provider_core::Message;
/// Access to existing settings and tracked transcript mutations.
/// # Implementors
/// Applications return their existing mutable fields and tracked iterator.
pub trait SessionAccess {
    /// Borrow managed-child settings without changing their values.
    fn settings(&mut self) -> Settings<'_>;
    /// Iterate mutable messages using the application's normal mutation tracking.
    fn messages_mut(&mut self) -> std::slice::IterMut<'_, Message>;
    /// Borrow the first message using the native tracked mutation operation.
    fn first_message_mut(&mut self) -> Option<&mut Message>;
    /// Set whether this session must retain its task-owned workspace.
    fn set_workspace_pinned(&mut self, pinned: bool);
    /// Invalidate the summary index after replacing an existing system message.
    fn reset_summary_index(&mut self);
    /// Append a message through the application's ordinary session method.
    fn add_message(&mut self, message: Message);
}
