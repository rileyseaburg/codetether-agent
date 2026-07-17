//! Capacity-stable append and oldest-byte eviction.

use super::OutputBuffer;

impl OutputBuffer {
    pub(in crate::mux::pty) fn append(&mut self, data: &[u8]) {
        self.mode.observe(data);
        self.base += self.bytes.append(data) as u64;
    }
}
