//! Session-store adapters and persistence lifecycle notifications.
use crate::{Restore, TailLoad};
use std::{future::Future, path::Path};
/// Supply native loading, listing and post-persistence index notifications.
pub trait Host {
    /// Session payload restored by the existing indexed store.
    type Session: Restore + Send + Sync;
    /// Load a full or bounded window from an already-resolved locator.
    /// # Errors
    /// Returns migration, storage or decoding errors.
    fn load_path(
        path: &Path,
        window: usize,
    ) -> impl Future<Output = anyhow::Result<TailLoad<Self::Session>>> + Send;
    /// List session identities newest first, optionally within a workspace.
    /// # Errors
    /// Returns the native listing failure.
    fn list_session_ids(
        workspace: Option<&Path>,
    ) -> impl Future<Output = anyhow::Result<Vec<String>>> + Send;
    /// Schedule recall indexing after a changed save.
    fn schedule_recall(session: &Self::Session);
    /// Notify proactive indexing after a changed save.
    fn notify_saved(session: &Self::Session);
    /// Schedule archival after a changed save.
    fn schedule_archive(session: &Self::Session);
    /// Notify proactive indexing before deletion.
    fn notify_removed(id: &str) -> impl Future<Output = ()> + Send;
    /// Remove recall records before deleting session storage.
    /// # Errors
    /// Returns the recall index removal failure.
    fn remove_recall(id: &str) -> impl Future<Output = anyhow::Result<()>> + Send;
}
