//! Constant-time mutation identities detect replacement by another clean vector.
use std::sync::atomic::{AtomicU64, Ordering};
pub(super) fn next() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}
pub(super) fn new() -> AtomicU64 {
    AtomicU64::new(next())
}
