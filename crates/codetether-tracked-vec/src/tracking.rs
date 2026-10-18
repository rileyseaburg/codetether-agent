//! Persistence dirtiness and constant-time mutation identity.
use crate::{TrackedVec, version};
use std::sync::atomic::Ordering;

impl<T> TrackedVec<T> {
    /// Return the earliest changed element, or `usize::MAX` when clean.
    /// # Examples
    /// ```rust
    /// let mut values = codetether_tracked_vec::TrackedVec::from(vec![1, 2]);
    /// values.clean();
    /// values.push(3);
    /// assert_eq!(values.dirty_from(), 2);
    /// ```
    pub fn dirty_from(&self) -> usize {
        self.dirty.load(Ordering::Relaxed)
    }
    /// Mark all current elements persisted without changing mutation identity.
    /// # Examples
    /// ```rust
    /// let values = codetether_tracked_vec::TrackedVec::from(vec![1]);
    /// let version = values.version();
    /// values.clean();
    /// assert_eq!(values.dirty_from(), usize::MAX);
    /// assert_eq!(values.version(), version);
    /// ```
    pub fn clean(&self) {
        self.dirty.store(usize::MAX, Ordering::Relaxed);
    }
    pub(crate) fn mark(&self, index: usize) {
        self.version.store(version::next(), Ordering::Relaxed);
        self.dirty.fetch_min(index, Ordering::Relaxed);
    }
    /// Return the mutation identity used to detect vector replacement.
    /// # Examples
    /// ```rust
    /// let mut values = codetether_tracked_vec::TrackedVec::from(vec![1]);
    /// let version = values.version();
    /// values.push(2);
    /// assert_ne!(values.version(), version);
    /// assert_eq!(values.clone().version(), values.version());
    /// ```
    pub fn version(&self) -> u64 {
        self.version.load(Ordering::Relaxed)
    }
}
