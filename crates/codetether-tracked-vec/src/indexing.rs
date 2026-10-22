//! Exact indexed mutations retain summaries before the changed suffix.
use crate::TrackedVec;
use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use std::slice::SliceIndex;

impl<T, I: SliceIndex<[T]>> Index<I> for TrackedVec<T> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.values[index]
    }
}

macro_rules! mutation_index {
    ($index:ty, $start:expr) => {
        impl<T> IndexMut<$index> for TrackedVec<T> {
            fn index_mut(&mut self, index: $index) -> &mut Self::Output {
                // Clamp the dirty marker so a caught bounds panic cannot leave
                // a marker beyond the end of the resident vector.
                let start = ($start)(&index).min(self.values.len());
                self.mark(start);
                &mut self.values[index]
            }
        }
    };
}
mutation_index!(usize, |index: &usize| *index);
mutation_index!(Range<usize>, |range: &Range<usize>| range.start);
mutation_index!(RangeFrom<usize>, |range: &RangeFrom<usize>| range.start);
mutation_index!(RangeInclusive<usize>, |range: &RangeInclusive<usize>| {
    *range.start()
});
mutation_index!(RangeFull, |_: &RangeFull| 0usize);
mutation_index!(RangeTo<usize>, |_: &RangeTo<usize>| 0usize);
mutation_index!(RangeToInclusive<usize>, |_: &RangeToInclusive<usize>| {
    0usize
});

#[cfg(test)]
mod tests {
    use super::*;
    fn clean() -> TrackedVec<u32> {
        let values = TrackedVec::from(vec![0, 1, 2, 3, 4]);
        values.clean();
        values
    }
    #[test]
    fn single_element_mutation_preserves_earlier_clean_records() {
        let mut values = clean();
        values[3] = 9;
        assert_eq!(values.dirty_from(), 3);
        assert_eq!(&values[..3], &[0, 1, 2]);
        values[1] = 8;
        assert_eq!(values.dirty_from(), 1);
    }
    #[test]
    fn range_mutations_track_their_actual_start() {
        let mut values = clean();
        values[2..4].fill(8);
        assert_eq!(values.dirty_from(), 2);
        let mut values = clean();
        values[2..].fill(8);
        assert_eq!(values.dirty_from(), 2);
        let mut values = clean();
        values[2..=3].fill(8);
        assert_eq!(values.dirty_from(), 2);
        let mut values = clean();
        values[..3].fill(8);
        assert_eq!(values.dirty_from(), 0);
        let mut values = clean();
        values[..=3].fill(8);
        assert_eq!(values.dirty_from(), 0);
        let mut values = clean();
        values[..].fill(8);
        assert_eq!(values.dirty_from(), 0);
    }
    #[test]
    fn arbitrary_vector_mutation_remains_conservative() {
        let mut values = clean();
        values.swap(3, 4);
        assert_eq!(values.dirty_from(), 0);
    }
}
