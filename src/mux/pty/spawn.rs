//! Platform selection for PTY creation and process startup.

use anyhow::Result;

use super::TerminalSize;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub(super) fn open(
    command: &str,
    workspace: &std::path::Path,
    size: TerminalSize,
    mux_session: &str,
) -> Result<(std::fs::File, std::process::Child)> {
    unix::open(command, workspace, size, mux_session)
}

#[cfg(not(unix))]
pub(super) fn open(
    _: &str,
    _: &std::path::Path,
    _: TerminalSize,
    _: &str,
) -> Result<(std::fs::File, std::process::Child)> {
    anyhow::bail!("server-owned PTYs are currently supported on Unix")
}
