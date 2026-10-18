//! Native storage and import boundaries for bounded session loading.
use super::LoadedSession;
use std::future::Future;
/// Native session loaders used in their existing fallback order.
/// # Implementors
/// Applications retain their durable snapshot and archive import implementation.
pub trait Host {
    /// Session type returned by both loading paths.
    type Session;
    /// Load the native session with a bounded history window.
    /// # Errors
    /// Returns the native storage failure used if archive import also fails.
    fn load_tail(
        id: &str,
        window: usize,
    ) -> impl Future<Output = anyhow::Result<LoadedSession<Self::Session>>>;
    /// Load or import a compatible archive session.
    /// # Errors
    /// Returns the archive failure appended to the native storage error.
    fn load_or_import(id: &str) -> impl Future<Output = anyhow::Result<Self::Session>>;
}
