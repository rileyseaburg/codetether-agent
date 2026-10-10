//! Hard-bounded, cancellable JPEG writer; failed attempts are cleared on drop.
use std::{
    io::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
};
use zeroize::Zeroizing;
const LIMIT: usize = 524_288;

pub(super) struct Output<'a> {
    pub(super) bytes: Zeroizing<Vec<u8>>,
    pub(super) too_large: bool,
    cancelled: &'a AtomicBool,
}
impl<'a> Output<'a> {
    pub(super) fn new(cancelled: &'a AtomicBool) -> Self {
        Self {
            bytes: Zeroizing::new(Vec::with_capacity(LIMIT)),
            too_large: false,
            cancelled,
        }
    }
}
impl Write for Output<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.cancelled.load(Ordering::Acquire) {
            return Err(io::Error::other("Capture cancelled"));
        }
        if bytes.len() > LIMIT - self.bytes.len() {
            self.too_large = true;
            return Err(io::Error::other("Screenshot exceeds the upload limit"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        if self.cancelled.load(Ordering::Acquire) {
            Err(io::Error::other("Capture cancelled"))
        } else {
            Ok(())
        }
    }
}
