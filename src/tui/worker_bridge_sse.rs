//! Byte-bounded SSE frame accumulation for worker tasks.

use super::IncomingTask;

const MAX_BUFFER_BYTES: usize = 1024 * 1024;

pub(super) struct SseBuffer {
    pending: String,
}

impl SseBuffer {
    pub(super) fn new() -> Self {
        Self {
            pending: String::new(),
        }
    }

    pub(super) fn push(&mut self, bytes: &[u8]) -> Result<Vec<IncomingTask>, Error> {
        let chunk = std::str::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)?;
        if self.pending.len().saturating_add(chunk.len()) > MAX_BUFFER_BYTES {
            self.pending.clear();
            return Err(Error::Oversized);
        }
        self.pending.push_str(chunk);
        let mut tasks = Vec::new();
        while let Some(pos) = self.pending.find("\n\n") {
            let remainder = self.pending.split_off(pos + 2);
            self.pending.truncate(pos);
            if let Some(task) = super::sse_event::parse(&self.pending) {
                tasks.push(task);
            }
            self.pending = remainder;
        }
        Ok(tasks)
    }
}

#[derive(Debug, thiserror::Error)]
pub(super) enum Error {
    #[error("SSE chunk is not valid UTF-8")]
    InvalidUtf8,
    #[error("SSE partial frame exceeded 1 MiB")]
    Oversized,
}

#[cfg(test)]
#[path = "worker_bridge_sse_tests.rs"]
mod tests;
