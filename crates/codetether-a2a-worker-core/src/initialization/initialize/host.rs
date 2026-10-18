use super::Options;
use std::{future::Future, sync::Arc};
/// Native dependencies used while constructing shared worker startup state.
/// # Implementors
/// Application adapters supply CLI settings, access policy, HTTP, bus and peers.
pub trait Host {
    /// Original arguments retained in the resulting context.
    type Args: Send + Sync;
    /// HTTP client cloned into task runtime state.
    type Client: Clone + Send + Sync;
    /// Shared bus retained by the worker and task runtime.
    type Bus: Send + Sync;
    /// Optional background peer whose lifetime follows the context.
    type Peer: Send;
    /// Bus announcement handle retained until initialization returns.
    type Announcement: Send;
    /// Apply native process-wide access policy before starting services.
    fn apply_access(args: &Self::Args);
    /// Borrow relevant settings from the native arguments.
    fn options(args: &Self::Args) -> Options<'_>;
    /// Construct the native HTTP client.
    /// # Errors
    /// Returns client configuration or initialization errors.
    fn client() -> anyhow::Result<Self::Client>;
    /// Create the bus and start its native archival sink.
    fn bus() -> Arc<Self::Bus>;
    /// Publish worker capabilities and tool schemas, returning the held handle.
    fn announce(bus: &Arc<Self::Bus>, worker_id: &str) -> Self::Announcement;
    /// Start the optional local peer after bus announcements.
    fn peer(
        args: &Self::Args,
        name: &str,
        bus: Arc<Self::Bus>,
    ) -> impl Future<Output = Option<Self::Peer>> + Send;
}
