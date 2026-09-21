//! Bounded little-endian GGUF header reader.
use anyhow::{Result, ensure};
use std::io::{Read, Seek};
pub(super) struct Reader<R> {
    pub inner: R,
    remaining: usize,
}
impl<R: Read + Seek> Reader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            remaining: 128 * 1024 * 1024,
        }
    }
    pub fn bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        ensure!(
            count <= self.remaining,
            "GGUF header exceeds its size budget"
        );
        self.remaining -= count;
        let mut bytes = vec![0; count];
        self.inner.read_exact(&mut bytes)?;
        Ok(bytes)
    }
    pub fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.bytes(4)?.try_into().expect("fixed length"),
        ))
    }
    pub fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(
            self.bytes(8)?.try_into().expect("fixed length"),
        ))
    }
    pub fn text(&mut self) -> Result<String> {
        let count = usize::try_from(self.u64()?)?;
        ensure!(count <= 4 * 1024 * 1024, "GGUF string is too large");
        Ok(String::from_utf8(self.bytes(count)?)?)
    }
}
