//! Side-effect-free validation before an MCP approval can be consumed.

use anyhow::{Result, bail};

pub(super) fn executable(command: &str) -> Result<()> {
    let command = command.trim();
    if command.is_empty() {
        bail!("MCP subprocess command must not be empty");
    }
    if which::which(command).is_err() {
        bail!("MCP subprocess executable not found: {command}");
    }
    Ok(())
}
