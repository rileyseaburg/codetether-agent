//! Platform cancellation for a mux-owned agent process.

use anyhow::{Result, bail};

#[cfg(unix)]
pub(super) fn send(pid: u32) -> Result<()> {
    // SAFETY: kill sends SIGINT to the validated child PID without dereferencing memory.
    if unsafe { libc::kill(-(pid as i32), libc::SIGINT) } == -1 {
        bail!(std::io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(not(unix))]
pub(super) fn send(_: u32) -> Result<()> {
    bail!("mux agent cancellation is not supported on this platform")
}
