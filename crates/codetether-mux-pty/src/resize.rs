//! Unix PTY resizing and child notification.

use std::fs::File;

use anyhow::{Result, bail};

use super::TerminalSize;

#[cfg(unix)]
pub(super) fn apply(master: &File, pid: u32, size: TerminalSize) -> Result<()> {
    use std::os::fd::AsRawFd;
    let value = libc::winsize {
        ws_row: size.rows,
        ws_col: size.columns,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: master is a valid PTY descriptor and value outlives ioctl.
    if unsafe { libc::ioctl(master.as_raw_fd(), libc::TIOCSWINSZ, &value) } == -1 {
        bail!(std::io::Error::last_os_error());
    }
    // SAFETY: the child created a process group whose id equals its pid.
    unsafe {
        libc::kill(-(pid as i32), libc::SIGWINCH);
    }
    Ok(())
}

#[cfg(not(unix))]
pub(super) fn apply(_: &File, _: u32, _: TerminalSize) -> Result<()> {
    bail!("PTY resizing is unsupported on this platform")
}
