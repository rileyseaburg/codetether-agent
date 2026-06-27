//! Bounded staging buffer for incoming SSE bytes.
//!
//! Caps the partial-frame accumulator so a peer that never emits a frame
//! delimiter cannot exhaust memory. When the cap is exceeded the buffer reports
//! [`StagingError::Overflow`] and the caller treats it as a protocol violation
//! (disconnect + reconnect). See `docs/transport-first-class-plan.md` Phase 2.

/// Error returned when the staging buffer would exceed its capacity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StagingError {
    /// A single un-delimited frame exceeded `max_bytes`.
    Overflow { max_bytes: usize },
}

/// A byte accumulator that refuses to grow past `max_bytes`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::staging::StagingBuffer;
///
/// let mut buf = StagingBuffer::new(8);
/// assert!(buf.extend(b"abc").is_ok());   // 3 bytes
/// assert!(buf.extend(b"defgh").is_ok()); // 3 + 5 = 8, exactly at cap
/// assert!(buf.extend(b"x").is_err());    // would exceed 8
/// ```
pub struct StagingBuffer {
    bytes: Vec<u8>,
    max_bytes: usize,
}

impl StagingBuffer {
    /// Create a buffer that rejects growth beyond `max_bytes`.
    pub fn new(max_bytes: usize) -> Self {
        Self { bytes: Vec::new(), max_bytes }
    }

    /// Append `chunk`, erroring if the total would exceed the cap.
    ///
    /// # Errors
    ///
    /// Returns [`StagingError::Overflow`] when `len() + chunk.len() > max_bytes`.
    pub fn extend(&mut self, chunk: &[u8]) -> Result<(), StagingError> {
        if self.bytes.len() + chunk.len() > self.max_bytes {
            return Err(StagingError::Overflow { max_bytes: self.max_bytes });
        }
        self.bytes.extend_from_slice(chunk);
        Ok(())
    }

    /// Mutable access to the underlying bytes for frame draining.
    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{StagingBuffer, StagingError};

    #[test]
    fn accepts_within_cap() {
        let mut buf = StagingBuffer::new(8);
        assert!(buf.extend(b"abcd").is_ok());
        assert!(buf.extend(b"efgh").is_ok());
        assert_eq!(buf.bytes_mut().len(), 8);
    }

    #[test]
    fn rejects_overflow() {
        let mut buf = StagingBuffer::new(4);
        assert_eq!(
            buf.extend(b"abcde"),
            Err(StagingError::Overflow { max_bytes: 4 })
        );
    }

    #[test]
    fn drain_lowers_occupancy_allowing_more() {
        let mut buf = StagingBuffer::new(4);
        buf.extend(b" abcd"[1..].as_ref()).unwrap();
        buf.bytes_mut().drain(..2);
        assert!(buf.extend(b"xy").is_ok());
    }
}
