//! Bounded replay storage for structured agent JSONL events.

const OUTPUT_LIMIT: usize = 4 * 1024 * 1024;
const READ_LIMIT: usize = 64 * 1024;

pub(super) struct TaskBuffer {
    base: u64,
    bytes: Vec<u8>,
}

impl TaskBuffer {
    pub(super) fn new() -> Self {
        Self {
            base: 0,
            bytes: Vec::new(),
        }
    }

    pub(super) fn append(&mut self, data: &[u8]) -> u64 {
        self.bytes.extend_from_slice(data);
        if self.bytes.len() > OUTPUT_LIMIT {
            let remove = self.bytes.len() - OUTPUT_LIMIT;
            self.bytes.drain(..remove);
            self.base += remove as u64;
        }
        self.base + self.bytes.len() as u64
    }

    pub(super) fn read(&self, offset: u64) -> (Vec<u8>, u64) {
        let start = offset.max(self.base);
        let skip = start.saturating_sub(self.base) as usize;
        let end = (skip + READ_LIMIT).min(self.bytes.len());
        let data = self.bytes.get(skip..end).unwrap_or_default().to_vec();
        (data.clone(), start + data.len() as u64)
    }
}
