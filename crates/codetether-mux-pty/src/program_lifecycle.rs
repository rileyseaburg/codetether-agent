//! Liveness, resizing and group termination for a PTY program.

use super::{TerminalSize, program::PtyProgram};
use anyhow::Result;
use std::sync::atomic::Ordering;

impl PtyProgram {
    pub(super) fn running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    pub(super) fn resize(&self, size: TerminalSize) -> Result<()> {
        super::resize::apply(&self.master.lock().unwrap(), self.pid, size)
    }

    pub(super) fn stop(&self) {
        #[cfg(unix)]
        if self.running() {
            // SAFETY: the live child created a process group whose id equals its pid.
            unsafe {
                libc::kill(-(self.pid as i32), libc::SIGHUP);
            }
        }
    }
}
