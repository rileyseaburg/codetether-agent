//! Construction and cloning retain explicit mutation identity.
use super::{TrackedVec, version};
use std::sync::atomic::{AtomicU64, AtomicUsize};
impl<T> Default for TrackedVec<T> {
    fn default() -> Self {
        Vec::new().into()
    }
}
impl<T> From<Vec<T>> for TrackedVec<T> {
    fn from(values: Vec<T>) -> Self {
        Self {
            values,
            dirty: AtomicUsize::new(0),
            version: version::new(),
        }
    }
}
impl<T: Clone> Clone for TrackedVec<T> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            dirty: AtomicUsize::new(self.dirty_from()),
            version: AtomicU64::new(self.version()),
        }
    }
}
