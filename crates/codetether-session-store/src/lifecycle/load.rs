//! Full and bounded loads with requested-identity validation.
use super::Host;
use crate::{Restore, Snapshot, TailLoad};
use anyhow::Result;
/// Load the complete session and normalize its sidecars.
///
/// # Arguments
/// * `id` — Requested durable session identity.
/// # Returns
/// The complete restored session.
/// # Errors
/// Identity, locator, migration, storage or decoding failures.
/// # Examples
/// ```no_run
/// # async fn run<H: codetether_session_store::lifecycle::Host>(id: &str) -> anyhow::Result<()> {
/// let _result = codetether_session_store::lifecycle::load::<H>(id).await?;
/// # Ok(())
/// # }
/// ```
pub async fn load<H: Host>(id: &str) -> Result<H::Session> {
    let path = super::session_path(id)?;
    let mut session = H::load_path(&path, usize::MAX).await?.session;
    anyhow::ensure!(
        session.id() == id,
        "Session file identity does not match requested ID"
    );
    session.normalize_sidecars();
    Ok(session)
}
/// Load a bounded session while checking its requested identity.
///
/// # Arguments
/// * `id` — Requested durable session identity.
/// * `window` — Maximum retained message and tool-use window.
/// # Returns
/// The restored window and omitted-entry metadata.
/// # Errors
/// Identity, locator, migration, storage or decoding failures.
/// # Examples
/// ```no_run
/// # async fn run<H: codetether_session_store::lifecycle::Host>(id: &str) -> anyhow::Result<()> {
/// let _result = codetether_session_store::lifecycle::load_tail::<H>(id, 100).await?;
/// # Ok(())
/// # }
/// ```
pub async fn load_tail<H: Host>(id: &str, window: usize) -> Result<TailLoad<H::Session>> {
    let path = super::session_path(id)?;
    let loaded = H::load_path(&path, window).await?;
    anyhow::ensure!(
        loaded.session.id() == id,
        "Session file identity does not match requested ID"
    );
    Ok(loaded)
}
