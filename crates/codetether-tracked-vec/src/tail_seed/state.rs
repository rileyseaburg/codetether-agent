//! Thread-local deserialization cap and discarded-entry accounting.
use std::cell::Cell;

thread_local! {
    pub(super) static TAIL_CAP: Cell<usize> = const { Cell::new(usize::MAX) };
    pub(super) static TAIL_DROPPED: Cell<usize> = const { Cell::new(0) };
}
