//! Capacity-stable append and oldest-byte eviction.

use super::{OUTPUT_LIMIT, OutputBuffer};

impl OutputBuffer {
    pub(in crate::mux::pty) fn append(&mut self, data: &[u8]) {
        self.mode.observe(data);
        if data.len() >= OUTPUT_LIMIT {
            self.base += (self.bytes.len() + data.len() - OUTPUT_LIMIT) as u64;
            self.bytes.clear();
            self.bytes.extend(&data[data.len() - OUTPUT_LIMIT..]);
            return;
        }
        let excess = self
            .bytes
            .len()
            .saturating_add(data.len())
            .saturating_sub(OUTPUT_LIMIT);
        self.bytes.drain(..excess);
        self.base += excess as u64;
        self.bytes.extend(data);
    }
}
