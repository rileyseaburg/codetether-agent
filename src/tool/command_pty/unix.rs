//! Unix `openpty` allocation and controlling-terminal setup.

use super::Attached;
use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd};
use std::process::Stdio;

pub(super) fn attach(command: &mut tokio::process::Command) -> io::Result<Attached> {
    let mut master = -1;
    let mut slave = -1;
    let size = libc::winsize {
        ws_row: 24,
        ws_col: 80,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // SAFETY: openpty initializes both owned file descriptors or returns an
    // error. The null termios pointer requests system defaults.
    if unsafe {
        libc::openpty(
            &mut master,
            &mut slave,
            std::ptr::null_mut(),
            std::ptr::null(),
            &size,
        )
    } == -1
    {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: successful openpty returned unique owned descriptors.
    let master = unsafe { File::from_raw_fd(master) };
    let slave = unsafe { File::from_raw_fd(slave) };
    let slave_fd = slave.as_raw_fd();
    command
        .stdin(Stdio::from(slave.try_clone()?))
        .stdout(Stdio::from(slave.try_clone()?))
        .stderr(Stdio::from(slave));
    // SAFETY: this runs after the existing setsid pre-exec hook; slave_fd is
    // inherited as child stdio and is valid until exec.
    unsafe {
        command.pre_exec(move || {
            (libc::ioctl(slave_fd, libc::TIOCSCTTY, 0) != -1)
                .then_some(())
                .ok_or_else(io::Error::last_os_error)
        });
    }
    Ok(Attached::Pty(master))
}
