//! Run indexed session restoration on a blocking database worker.
use crate::{Restore, TailLoad};
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::path::Path;

/// Restore an indexed session window with its original durable identity.
/// # Arguments
/// `path` selects the locator, `window` limits each record stream, and
/// `prepare` performs the application's legacy import before indexed reading.
/// # Returns
/// The restored session, excluded record count and locator byte length.
/// # Errors
/// Propagates recovery, import, SQLite, decoding, identity and file errors.
/// Blocking-worker failures retain the session reader context.
/// # Panics
/// Requires a Tokio runtime for its blocking worker.
/// # Examples
/// ```rust,no_run
/// # async fn example<S: codetether_session_store::Restore + Send + 'static>(path: &std::path::Path) -> anyhow::Result<()>
/// # where S::Tool: serde::de::DeserializeOwned {
/// let loaded = codetether_session_store::load::<S>(path, 20, |_| Ok(())).await?;
/// # Ok(()) }
/// ```
pub async fn load<S: Restore + Send + 'static>(
    path: &Path,
    window: usize,
    prepare: impl FnOnce(&Path) -> Result<()> + Send + 'static,
) -> Result<TailLoad<S>>
where
    S::Tool: DeserializeOwned,
{
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || super::load_sync::load_sync::<S>(&path, window, prepare))
        .await
        .context("session reader panicked")?
}
