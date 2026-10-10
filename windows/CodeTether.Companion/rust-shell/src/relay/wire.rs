//! Fixed-capacity JSON request storage, erased after the last HTTP body owner.
use super::Error;
use serde::Serialize;
use std::io::{self, Write};
use zeroize::Zeroizing;
const LIMIT: usize = 710_000;
struct Wire(Zeroizing<Vec<u8>>);
impl AsRef<[u8]> for Wire {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl Write for Wire {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > LIMIT - self.0.len() {
            return Err(io::Error::other("Frame request too large"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
pub(super) fn body<T: Serialize>(value: &T) -> Result<reqwest::Body, Error> {
    let mut wire = Wire(Zeroizing::new(Vec::with_capacity(LIMIT)));
    serde_json::to_writer(&mut wire, value).map_err(|_| Error::InvalidInput)?;
    Ok(reqwest::Body::from(bytes::Bytes::from_owner(wire)))
}
