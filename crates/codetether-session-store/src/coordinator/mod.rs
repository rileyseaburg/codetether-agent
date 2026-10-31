//! Bounded FIFO writes, post-commit replies, and cross-process exclusion.
//! Workers live until process exit. Accepted writes continue after cancellation;
//! retry with the operation's durable identity, not an invented replacement.
mod drain;
mod error;
mod job;
mod lock;
mod registry;
mod worker;
use anyhow::Result;
pub use error::CoordinationError;
use rusqlite::Connection;
use std::path::Path;

/// Submit a synchronous write; overload fails rather than growing the queue.
/// Returns the operation result after execution, or a typed admission/worker error.
/// Never call recursively from an operation; use its supplied connection.
/// # Examples
/// ```no_run
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// codetether_session_store::coordinator::write(path, |_db| Ok(()))?;
/// # Ok(()) }
/// ```
pub fn write<T: Send + 'static>(
    path: &Path,
    operation: impl FnOnce(&mut Connection) -> Result<T> + Send + 'static,
) -> Result<T> {
    let writer = registry::get(path)?;
    let (send, receive) = std::sync::mpsc::channel();
    writer.submit(job::new(operation, move |result| {
        let _ = send.send(result);
    }))?;
    receive.recv().map_err(|_| CoordinationError::Stopped)?
}

/// Submit a write without occupying Tokio's blocking pool while awaiting it.
/// Queue admission is bounded; cancellation does not revoke an accepted write.
/// # Examples
/// ```no_run
/// # async fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// codetether_session_store::coordinator::write_async(path, |_db| Ok(())).await?;
/// # Ok(()) }
/// ```
pub async fn write_async<T: Send + 'static>(
    path: &Path,
    operation: impl FnOnce(&mut Connection) -> Result<T> + Send + 'static,
) -> Result<T> {
    let writer = registry::get(path)?;
    let (send, receive) = tokio::sync::oneshot::channel();
    writer.submit(job::new(operation, move |result| {
        let _ = send.send(result);
    }))?;
    receive.await.map_err(|_| CoordinationError::Stopped)?
}

/// Initialize schema through the writer once before opening a reader.
/// # Examples
/// ```no_run
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let reader = codetether_session_store::connection::open(path)?;
/// # Ok(()) }
/// ```
pub(crate) fn initialize(path: &Path) -> Result<()> {
    let writer = registry::get(path)?;
    if !writer.ready.load(std::sync::atomic::Ordering::Acquire) {
        write(path, |_db| Ok(()))?;
    }
    Ok(())
}
