//! Input, output, resize, and lifecycle operations for a PTY program.

use std::io::Write;
use std::sync::atomic::Ordering;

use anyhow::Result;

use super::{PtyChunk, TerminalSize, program::PtyProgram};

impl PtyProgram {
    pub(super) fn input(&self, data: &[u8]) -> Result<()> {
        self.master.lock().unwrap().write_all(data)?;
        Ok(())
    }

    pub(super) fn read(&self, offset: u64) -> PtyChunk {
        let (data, next_offset) = self.output.lock().unwrap().read(offset);
        PtyChunk {
            data,
            next_offset,
            running: self.running(),
        }
    }

    pub(super) fn earliest(&self) -> u64 {
        self.output.lock().unwrap().earliest()
    }

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
