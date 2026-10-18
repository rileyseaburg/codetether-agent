//! Capacity-stable append and oldest-byte eviction.

use super::OutputBuffer;

impl OutputBuffer {
    /// Append output, observe terminal modes, and evict oldest bytes as needed.
    pub fn append(&mut self, data: &[u8]) {
        self.mode.observe(data);
        self.base += self.bytes.append(data) as u64;
    }
}
