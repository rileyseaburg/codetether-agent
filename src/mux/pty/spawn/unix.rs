//! Unix `openpty` setup and controlling-terminal assignment.

use std::fs::File;
use std::os::fd::FromRawFd;
use std::process::Child;

use anyhow::{Result, bail};

use super::super::TerminalSize;

mod command;
mod control;

pub(super) fn open(
    command: &str,
    workspace: &std::path::Path,
    size: TerminalSize,
    mux_session: &str,
) -> Result<(File, Child)> {
    let mut master = -1;
    let mut slave = -1;
    let dimensions = libc::winsize {
        ws_row: size.rows,
        ws_col: size.columns,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: openpty initializes both owned file descriptors on success.
    if unsafe {
        libc::openpty(
            &mut master,
            &mut slave,
            std::ptr::null_mut(),
            std::ptr::null(),
            &dimensions,
        )
    } == -1
    {
        bail!(std::io::Error::last_os_error());
    }
    // SAFETY: successful openpty returned unique owned descriptors.
    let master = unsafe { File::from_raw_fd(master) };
    let slave = unsafe { File::from_raw_fd(slave) };
    let mut process = command::configured(command, workspace, slave, mux_session)?;
    control::assign(&mut process);
    Ok((master, process.spawn()?))
}
