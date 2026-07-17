//! Per-poll output budgeting with explicit omission accounting.

pub(super) struct Buffer {
    bytes: Vec<u8>,
    max_bytes: usize,
    omitted: usize,
}

impl Buffer {
    pub fn new(max_bytes: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(max_bytes.min(64 * 1024)),
            max_bytes,
            omitted: 0,
        }
    }

    pub fn push(&mut self, chunk: &[u8]) {
        let remaining = self.max_bytes.saturating_sub(self.bytes.len());
        let shown = remaining.min(chunk.len());
        self.bytes.extend_from_slice(&chunk[..shown]);
        self.omitted += chunk.len() - shown;
    }

    pub fn finish(self) -> (String, usize) {
        let mut output = String::from_utf8_lossy(&self.bytes).into_owned();
        if self.omitted > 0 {
            output.push_str(&format!("\n... {} bytes omitted ...", self.omitted));
        }
        (output, self.omitted)
    }
}
