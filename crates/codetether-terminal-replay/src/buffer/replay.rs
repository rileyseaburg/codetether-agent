//! Fixed-capacity byte ring used by the PTY replay buffer.

pub(super) struct ReplayBytes {
    pub(super) storage: Vec<u8>,
    pub(super) head: usize,
}

impl ReplayBytes {
    pub(super) fn new() -> Self {
        Self {
            storage: Vec::new(),
            head: 0,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.storage.len()
    }

    #[cfg(test)]
    pub(super) fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    pub(super) fn read(&self, skip: usize, limit: usize) -> Vec<u8> {
        let count = self.len().saturating_sub(skip).min(limit);
        if count == 0 {
            return Vec::new();
        }
        let start = (self.head + skip) % self.len();
        let first = (self.len() - start).min(count);
        let mut output = Vec::with_capacity(count);
        output.extend_from_slice(&self.storage[start..start + first]);
        output.extend_from_slice(&self.storage[..count - first]);
        output
    }
}
