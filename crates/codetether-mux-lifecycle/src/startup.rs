//! Duplicate detection and bounded mux startup discovery.

use anyhow::{Result, bail};

use codetether_mux_registry::SessionTarget;

/// Reject duplicate session names, including stale registry records.
///
/// # Arguments
/// name is the proposed session name.
/// # Returns
/// Unit when no matching record exists.
/// # Errors
/// Returns duplicate/stale record errors or registry lookup errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_lifecycle::startup::reject_duplicate("work").await?;
/// # Ok(()) }
/// ```
pub async fn reject_duplicate(name: &str) -> Result<()> {
    let Some(target) = codetether_mux_registry::find_session(name).await? else {
        return Ok(());
    };
    if codetether_mux_client::handshake::probe(&target.record)
        .await
        .is_ok()
    {
        bail!("mux session '{name}' already exists");
    }
    bail!("mux session '{name}' has a stale registry record; inspect it before removing it")
}

/// Wait for a freshly spawned server to publish its initial session.
///
/// # Arguments
/// name identifies the expected session; child is the daemon process.
/// # Returns
/// The first reachable registry target for the new session.
/// # Errors
/// Returns early child exit, process polling or startup timeout errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let mut child = codetether_mux_lifecycle::spawn::command("work", std::path::Path::new("."), "bootstrap", codetether_mux_workspace::Isolation::Shared)?.spawn()?;
/// let target = codetether_mux_lifecycle::startup::wait_for_record("work", &mut child).await?;
/// # Ok(()) }
/// ```
pub async fn wait_for_record(name: &str, child: &mut std::process::Child) -> Result<SessionTarget> {
    let mut backoff = crate::backoff::Backoff::new();
    while backoff.remaining() {
        if let Some(status) = child.try_wait()? {
            bail!("mux server exited during startup: {status}");
        }
        if let Ok(Some(target)) = codetether_mux_registry::find_session(name).await
            && codetether_mux_client::MuxConnection::connect(&target)
                .await
                .is_ok()
        {
            return Ok(target);
        }
        backoff.sleep().await;
    }
    let _ = child.kill();
    bail!("timed out waiting for mux server startup")
}
