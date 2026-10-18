//! Mutable vector borrows explicitly invalidate the loaded suffix.
use super::TrackedVec;
use std::ops::{Deref, DerefMut};

impl<T> Deref for TrackedVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
impl<T> DerefMut for TrackedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mark(0);
        &mut self.values
    }
}
impl<T: PartialEq> PartialEq for TrackedVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}
impl<T: PartialEq> PartialEq<Vec<T>> for TrackedVec<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.values == *other
    }
}
