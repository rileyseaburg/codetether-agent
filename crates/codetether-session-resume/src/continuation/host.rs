//! Session identity and durable child-owner bindings for continuation.
use std::future::Future;
/// Native session state required for bounded-history continuation.
/// # Implementors
/// Applications retain their storage-mode check and child-owner persistence.
pub trait Session {
    /// Whether the loaded session already has durable storage ownership.
    fn is_persisted(&self) -> bool;
    /// Current session identity.
    fn id(&self) -> &str;
    /// Current optional title.
    fn title(&self) -> Option<&str>;
    /// Transfer durable child ownership before changing the session identity.
    /// # Errors
    /// Returns the existing child-owner persistence failure.
    fn reparent_owner(
        original: &str,
        continuation: &str,
    ) -> impl Future<Output = anyhow::Result<()>>;
    /// Set the continuation identity after ownership transfer succeeds.
    fn set_id(&mut self, id: String);
    /// Set the continuation title.
    fn set_title(&mut self, title: String);
}
