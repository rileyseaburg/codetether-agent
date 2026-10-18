//! Local tmux-style `Ctrl+B`, `D` detach sequence filtering.

pub(super) struct Detector {
    prefix: bool,
}

/// Forwardable terminal bytes and an attachment-detach decision.
/// # Examples
/// ```
/// let filtered = codetether_mux_input::Filtered { data: b"ls".to_vec(), detach: false };
/// assert!(!filtered.detach);
/// ```
pub struct Filtered {
    /// Bytes to send to the server-owned terminal.
    pub data: Vec<u8>,
    /// Whether this attachment should detach after sending the bytes.
    pub detach: bool,
}

impl Detector {
    pub(super) fn new() -> Self {
        Self { prefix: false }
    }

    pub(super) fn filter(&mut self, bytes: &[u8]) -> Filtered {
        let mut data = Vec::with_capacity(bytes.len());
        for &byte in bytes {
            if self.prefix {
                self.prefix = false;
                if matches!(byte, b'd' | b'D') {
                    return Filtered { data, detach: true };
                }
                data.extend([2, byte]);
            } else if byte == 2 {
                self.prefix = true;
            } else {
                data.push(byte);
            }
        }
        Filtered {
            data,
            detach: false,
        }
    }
}
#[cfg(test)]
#[path = "detach_tests.rs"]
mod tests;
