//! Input, output, resize, and lifecycle operations for a PTY program.

use std::io::Write;

use anyhow::Result;

use super::{PtyAttach, PtyChunk, program::PtyProgram};

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

    pub(super) fn attach_state(&self) -> PtyAttach {
        let (offset, replay_until, alternate_screen) = self.output.lock().unwrap().attach_state();
        PtyAttach {
            offset,
            replay_until,
            alternate_screen,
        }
    }
}
