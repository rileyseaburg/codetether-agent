//! Exact mux session restart primitives.

use std::path::Path;

use anyhow::{Result, bail};

/// Wait until no live server hosts `name`.
pub(super) async fn wait_stopped(name: &str) -> Result<()> {
    for _ in 0..100 {
        if crate::mux::registry::find_session(name).await?.is_none() {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    bail!("mux session did not stop cleanly")
}

/// Recreate `name` at exactly `workspace`, joining the workspace's server if it
/// is still running (other sessions kept it alive) or starting a fresh one.
pub(super) async fn start_exact(
    name: &str,
    workspace: &Path,
    isolation: crate::mux::isolation::Isolation,
) -> Result<()> {
    crate::mux::registry::validate_name(name)?;
    if let Some(record) = super::start_join::live_server(workspace).await {
        super::start_join::create_session(record, name, workspace.to_path_buf()).await?;
        return Ok(());
    }
    let token = crate::mux::token::generate();
    let mut process = crate::mux::command::spawn::command(name, workspace, &token, isolation)?;
    let mut child = process
        .spawn()
        .map_err(|error| anyhow::anyhow!("restart mux server: {error}"))?;
    crate::mux::command::startup::wait_for_record(name, &mut child).await?;
    Ok(())
}
