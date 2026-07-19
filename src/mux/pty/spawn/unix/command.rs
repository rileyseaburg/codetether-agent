//! Child command construction for a mux-owned Unix PTY.

use anyhow::Result;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

pub(super) fn configured(
    command: &str,
    workspace: &Path,
    slave: File,
    mux_session: &str,
) -> Result<Command> {
    let shell = std::env::var_os("SHELL").unwrap_or_else(|| "/bin/sh".into());
    let mut process = Command::new(shell);
    process
        .arg("-lc")
        .arg(command)
        .current_dir(workspace)
        .env(crate::mux::coordination::SESSION_ENV, mux_session)
        .stdin(Stdio::from(slave.try_clone()?))
        .stdout(Stdio::from(slave.try_clone()?))
        .stderr(Stdio::from(slave));
    Ok(process)
}

#[cfg(test)]
mod tests;
