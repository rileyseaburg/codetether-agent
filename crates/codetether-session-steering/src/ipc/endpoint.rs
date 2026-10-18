//! Active session endpoint lifecycle.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use tokio::net::UnixListener;

/// Owns one session socket and its background listener.
/// Dropping the endpoint aborts its listener and removes its socket path.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// let endpoint = codetether_session_steering::Endpoint::start("session-1234")?;
/// drop(endpoint);
/// # Ok(()) }
/// ```
pub struct Endpoint {
    path: PathBuf,
    handle: tokio::task::JoinHandle<()>,
}

impl Endpoint {
    /// Bind the local endpoint for a durable session.
    /// # Arguments
    /// `session_id` is a valid durable session identity.
    /// # Returns
    /// An endpoint guard owning the listener and socket path.
    /// # Errors
    /// Returns errors for an invalid identity, directory or socket I/O, or an
    /// already-active owner at the same address.
    /// # Panics
    /// Panics when invoked outside a Tokio runtime with I/O enabled.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let endpoint = codetether_session_steering::Endpoint::start("session-1234")?;
    /// # Ok(()) }
    /// ```
    pub fn start(session_id: &str) -> Result<Self> {
        Self::start_at(session_id, super::path::for_session(session_id)?)
    }

    pub(super) fn start_at(session_id: &str, path: PathBuf) -> Result<Self> {
        clear_stale(&path)?;
        let listener = UnixListener::bind(&path)
            .with_context(|| format!("bind session endpoint {}", path.display()))?;
        let handle = tokio::spawn(super::server::run(listener, session_id.to_string()));
        Ok(Self { path, handle })
    }
}

fn clear_stale(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if std::os::unix::net::UnixStream::connect(path).is_ok() {
        bail!("session already has an active owner");
    }
    std::fs::remove_file(path)?;
    Ok(())
}

impl Drop for Endpoint {
    fn drop(&mut self) {
        self.handle.abort();
        let _ = std::fs::remove_file(&self.path);
    }
}
