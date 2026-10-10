//! Cooperative capture cancellation; flags are one-way and per operation.
use crate::Error;
use std::sync::atomic::{AtomicBool, Ordering};

pub(super) fn check(cancelled: &AtomicBool) -> Result<(), Error> {
    if cancelled.load(Ordering::Acquire) {
        Err(Error::Cancelled)
    } else {
        Ok(())
    }
}
