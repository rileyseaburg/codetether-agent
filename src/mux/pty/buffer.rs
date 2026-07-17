//! Bounded replay buffer for reconnecting PTY clients.

use super::terminal_mode::TerminalMode;

mod append;
#[cfg(test)]
mod benchmark;
mod replay;
mod replay_append;
#[cfg(test)]
mod tests;

use replay::ReplayBytes;

const OUTPUT_LIMIT: usize = 4 * 1024 * 1024;
const READ_LIMIT: usize = 64 * 1024;

pub(super) struct OutputBuffer {
    base: u64,
    bytes: ReplayBytes,
    mode: TerminalMode,
}

impl OutputBuffer {
    pub(super) fn new() -> Self {
        Self {
            base: 0,
            bytes: ReplayBytes::new(),
            mode: TerminalMode::new(false),
        }
    }

    pub(super) fn read(&self, offset: u64) -> (Vec<u8>, u64) {
        let start = offset.max(self.base);
        let skip = start.saturating_sub(self.base) as usize;
        let data = self.bytes.read(skip, READ_LIMIT);
        let next = start + data.len() as u64;
        (data, next)
    }

    pub(super) fn earliest(&self) -> u64 {
        self.base
    }

    pub(super) fn attach_state(&self) -> (u64, bool) {
        (self.base, self.mode.active())
    }
}
