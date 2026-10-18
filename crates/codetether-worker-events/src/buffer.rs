//! Byte-bounded SSE frame accumulation for worker tasks.

use crate::{Error, IncomingTask};

const MAX_BUFFER_BYTES: usize = 1024 * 1024;

/// Bounded incremental SSE task decoder.
/// # Examples
/// ```
/// let mut buffer = codetether_worker_events::SseBuffer::new();
/// assert!(buffer.push(b"data: [DONE]\n\n").unwrap().is_empty());
/// ```
#[derive(Default)]
pub struct SseBuffer {
    pending: String,
}

impl SseBuffer {
    /// Construct an empty incremental decoder.
    pub fn new() -> Self {
        Self {
            pending: String::new(),
        }
    }

    /// Add a UTF-8 chunk and return tasks from complete double-newline frames.
    /// # Arguments
    /// * `bytes` - One chunk of the worker event stream.
    /// # Returns
    /// Parsed tasks in frame order; incomplete text remains buffered.
    /// # Errors
    /// Returns invalid UTF-8 or buffer overflow errors. Overflow clears the buffer.
    /// # Examples
    /// ```
    /// assert!(codetether_worker_events::SseBuffer::new().push(&[0xff]).is_err());
    /// ```
    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<IncomingTask>, Error> {
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
            if let Some(task) = crate::event::parse(&self.pending) {
                tasks.push(task);
            }
            self.pending = remainder;
        }
        Ok(tasks)
    }
}

#[cfg(test)]
#[path = "buffer_tests.rs"]
mod tests;
