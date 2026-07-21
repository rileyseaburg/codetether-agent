//! Exact mux server restart primitives.

use std::path::Path;

use anyhow::{Context, Result, bail};

pub(super) async fn wait_stopped(name: &str) -> Result<()> {
    for _ in 0..100 {
        if crate::mux::registry::load(name).await.is_err() {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    bail!("mux session did not stop cleanly")
}

pub(super) async fn start_exact(name: &str, workspace: &Path) -> Result<()> {
    crate::mux::registry::validate_name(name)?;
    let token = crate::mux::token::generate();
    let mut process = crate::mux::command::spawn::command(name, workspace, &token)?;
    let mut child = process.spawn().context("restart mux server")?;
    crate::mux::command::startup::wait_for_record(name, &mut child).await?;
    Ok(())
}
