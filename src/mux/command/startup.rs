//! Duplicate detection and bounded mux startup discovery.

use anyhow::{Result, bail};

pub(in crate::mux) async fn reject_duplicate(name: &str) -> Result<()> {
    let Ok(record) = crate::mux::registry::load(name).await else {
        return Ok(());
    };
    if crate::mux::client::MuxConnection::connect(&record)
        .await
        .is_ok()
    {
        bail!("mux session '{name}' already exists");
    }
    bail!("mux session '{name}' has a stale registry record; inspect it before removing it")
}

pub(in crate::mux) async fn wait_for_record(
    name: &str,
    child: &mut std::process::Child,
) -> Result<crate::mux::registry::MuxRecord> {
    for _ in 0..100 {
        if let Some(status) = child.try_wait()? {
            bail!("mux server exited during startup: {status}");
        }
        if let Ok(record) = crate::mux::registry::load(name).await
            && crate::mux::client::MuxConnection::connect(&record)
                .await
                .is_ok()
        {
            return Ok(record);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    let _ = child.kill();
    bail!("timed out waiting for mux server startup")
}
