//! Duplicate detection and bounded mux startup discovery.

use anyhow::{Result, bail};

use crate::mux::registry::SessionTarget;

/// Fail when a live server already hosts a session called `name`.
pub(in crate::mux) async fn reject_duplicate(name: &str) -> Result<()> {
    let Some(target) = crate::mux::registry::find_session(name).await? else {
        return Ok(());
    };
    if crate::mux::client::probe(&target.record).await.is_ok() {
        bail!("mux session '{name}' already exists");
    }
    bail!("mux session '{name}' has a stale registry record; inspect it before removing it")
}

/// Wait for a freshly spawned server to publish the session it was started with.
pub(in crate::mux) async fn wait_for_record(
    name: &str,
    child: &mut std::process::Child,
) -> Result<SessionTarget> {
    for _ in 0..100 {
        if let Some(status) = child.try_wait()? {
            bail!("mux server exited during startup: {status}");
        }
        if let Ok(Some(target)) = crate::mux::registry::find_session(name).await
            && crate::mux::client::MuxConnection::connect(&target)
                .await
                .is_ok()
        {
            return Ok(target);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    let _ = child.kill();
    bail!("timed out waiting for mux server startup")
}
