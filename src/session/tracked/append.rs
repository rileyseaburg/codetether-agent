//! Append and truncate retain an exact dirty suffix.
use super::TrackedVec;

impl<T> TrackedVec<T> {
    /// Append one element without invalidating earlier records.
    pub fn push(&mut self, value: T) {
        self.mark(self.values.len());
        self.values.push(value);
    }
    pub(crate) fn evict(&mut self, count: usize) {
        self.values.drain(..count);
        if self.dirty_from() != usize::MAX {
            self.dirty
                .fetch_sub(count, std::sync::atomic::Ordering::Relaxed);
        }
    }
    /// Append an iterator without invalidating earlier records.
    pub fn extend(&mut self, values: impl IntoIterator<Item = T>) {
        self.mark(self.values.len());
        self.values.extend(values);
    }
    /// Remove a suffix; persistence retains the preceding records.
    pub fn truncate(&mut self, len: usize) {
        if len < self.values.len() {
            self.mark(len);
            self.values.truncate(len);
        }
    }
    /// Explicitly clear this loaded vector.
    pub fn clear(&mut self) {
        self.mark(0);
        self.values.clear();
    }
    /// Consume the vector, returning its owned values.
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}
impl<T> FromIterator<T> for TrackedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}
