//! Bounded replay buffer for reconnecting PTY clients.

use super::terminal_mode::TerminalMode;

mod append;
mod replay;
mod replay_append;
#[cfg(test)]
#[path = "buffer/test_suite.rs"]
mod tests;

use replay::ReplayBytes;

const OUTPUT_LIMIT: usize = 4 * 1024 * 1024;
const READ_LIMIT: usize = 64 * 1024;
const ATTACH_REPLAY_LIMIT: u64 = READ_LIMIT as u64;

/// Fixed-capacity output history with absolute read and attachment offsets.
/// # Examples
/// ```
/// let mut buffer = codetether_terminal_replay::OutputBuffer::new();
/// buffer.append(b"hello");
/// assert_eq!(buffer.read(0), (b"hello".to_vec(), 5));
/// ```
pub struct OutputBuffer {
    base: u64,
    bytes: ReplayBytes,
    mode: TerminalMode,
}

impl OutputBuffer {
    /// Create an empty buffer on the normal terminal screen.
    pub fn new() -> Self {
        Self {
            base: 0,
            bytes: ReplayBytes::new(),
            mode: TerminalMode::new(false),
        }
    }

    /// Read at most 64 KiB from the available history, clamping old offsets.
    pub fn read(&self, offset: u64) -> (Vec<u8>, u64) {
        let start = offset.max(self.base);
        let skip = start.saturating_sub(self.base) as usize;
        let data = self.bytes.read(skip, READ_LIMIT);
        let next = start + data.len() as u64;
        (data, next)
    }

    /// Return the earliest retained absolute byte offset.
    pub fn earliest(&self) -> u64 {
        self.base
    }

    /// Return the absolute byte offset following the newest output.
    pub fn latest(&self) -> u64 {
        self.base + self.bytes.len() as u64
    }

    /// Return the bounded replay start, replay end and alternate-screen state.
    pub fn attach_state(&self) -> (u64, u64, bool) {
        let latest = self.latest();
        let offset = latest.saturating_sub(ATTACH_REPLAY_LIMIT).max(self.base);
        (offset, latest, self.mode.active())
    }
}
