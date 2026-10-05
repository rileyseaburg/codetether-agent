//! Mutation-aware vectors for delta persistence. Read access never dirties history.
//! [`TrackedVec`] retains vector indexing and JSON serialization.
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
mod access;
mod conversion;
mod deserialize;
mod version;
pub(crate) use deserialize::tail;
mod append;
mod iteration;

/// A vector that records its earliest changed element.
///
/// # Examples
/// ```
/// use codetether_agent::session::tracked::TrackedVec;
/// let mut values = TrackedVec::from(vec![1]);
/// values.push(2);
/// assert_eq!(values.as_slice(), &[1, 2]);
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrackedVec<T> {
    values: Vec<T>,
    #[serde(skip)]
    dirty: AtomicUsize,
    #[serde(skip, default = "version::new")]
    version: AtomicU64,
}
impl<T> TrackedVec<T> {
    pub(crate) fn dirty_from(&self) -> usize {
        self.dirty.load(Ordering::Relaxed)
    }
    pub(crate) fn clean(&self) {
        self.dirty.store(usize::MAX, Ordering::Relaxed);
    }
    fn mark(&self, index: usize) {
        self.version.store(version::next(), Ordering::Relaxed);
        self.dirty.fetch_min(index, Ordering::Relaxed);
    }
    pub(crate) fn version(&self) -> u64 {
        self.version.load(Ordering::Relaxed)
    }
}
