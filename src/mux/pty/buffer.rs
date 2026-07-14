//! Bounded replay buffer for reconnecting PTY clients.

use std::collections::VecDeque;

const OUTPUT_LIMIT: usize = 4 * 1024 * 1024;
const READ_LIMIT: usize = 64 * 1024;

pub(super) struct OutputBuffer {
    base: u64,
    bytes: VecDeque<u8>,
}

impl OutputBuffer {
    pub(super) fn new() -> Self {
        Self {
            base: 0,
            bytes: VecDeque::new(),
        }
    }

    pub(super) fn append(&mut self, data: &[u8]) {
        self.bytes.extend(data);
        let excess = self.bytes.len().saturating_sub(OUTPUT_LIMIT);
        self.bytes.drain(..excess);
        self.base += excess as u64;
    }

    pub(super) fn read(&self, offset: u64) -> (Vec<u8>, u64) {
        let start = offset.max(self.base);
        let skip = start.saturating_sub(self.base) as usize;
        let data = self
            .bytes
            .iter()
            .skip(skip)
            .take(READ_LIMIT)
            .copied()
            .collect::<Vec<_>>();
        let next = start + data.len() as u64;
        (data, next)
    }

    pub(super) fn earliest(&self) -> u64 {
        self.base
    }
}
