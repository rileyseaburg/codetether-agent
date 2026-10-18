//! Session history and storage access used by interactive undo.
use chrono::{DateTime, Utc};
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
use std::future::Future;
/// Mutable history and durable storage operations for undo.
/// # Implementors
/// Applications expose their existing message/page vectors and save operation.
pub trait Session {
    /// Borrow the message history and its aligned page metadata together.
    fn history(&mut self) -> (&mut Vec<Message>, &mut Vec<PageKind>);
    /// Record the time of a successful history truncation.
    fn set_updated_at(&mut self, timestamp: DateTime<Utc>);
    /// Persist the modified session.
    /// # Errors
    /// Returns the application's existing storage failure.
    fn save(&self) -> impl Future<Output = anyhow::Result<()>>;
}
