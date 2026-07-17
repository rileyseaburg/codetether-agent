//! Constant-time chunk insertion and oldest-byte replacement.

use super::{OUTPUT_LIMIT, replay::ReplayBytes};

impl ReplayBytes {
    pub(super) fn append(&mut self, data: &[u8]) -> usize {
        if data.len() >= OUTPUT_LIMIT {
            let evicted = self.len() + data.len() - OUTPUT_LIMIT;
            self.storage.clear();
            self.storage
                .extend_from_slice(&data[data.len() - OUTPUT_LIMIT..]);
            self.head = 0;
            return evicted;
        }
        let available = OUTPUT_LIMIT - self.len();
        let retained = available.min(data.len());
        self.storage.extend_from_slice(&data[..retained]);
        let overflow = &data[retained..];
        self.overwrite(overflow);
        overflow.len()
    }

    fn overwrite(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let first = (OUTPUT_LIMIT - self.head).min(data.len());
        self.storage[self.head..self.head + first].copy_from_slice(&data[..first]);
        self.storage[..data.len() - first].copy_from_slice(&data[first..]);
        self.head = (self.head + data.len()) % OUTPUT_LIMIT;
    }
}
