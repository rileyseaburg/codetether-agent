//! Temporarily select the bounded-loading cap for this thread.
use super::state::{TAIL_CAP, TAIL_DROPPED};
use std::cell::Cell;

/// Run `f` with the tail cap set to `cap` for the current thread. Returns
/// `(result, dropped)` where `dropped` is the total number of elements
/// discarded across all tail-capped sequences during the call.
/// # Arguments
/// `cap` selects the retained tail length; `f` performs synchronous deserialization.
/// # Returns
/// The closure output and total dropped entries, restoring the previous cap
/// and dropped count after the closure returns normally.
/// # Panics
/// Propagates closure panics; cap restoration is performed only on normal return.
/// # Examples
/// ```rust
/// let (value, dropped) = codetether_tracked_vec::tail_seed::with_tail_cap(2, || 42);
/// assert_eq!((value, dropped), (42, 0));
/// ```
pub fn with_tail_cap<R>(cap: usize, f: impl FnOnce() -> R) -> (R, usize) {
    TAIL_CAP.with(|c| {
        let prev_cap = c.get();
        c.set(cap);
        let dropped_before = TAIL_DROPPED.with(|d| {
            let v = d.get();
            d.set(0);
            v
        });
        let r = f();
        let dropped = TAIL_DROPPED.with(Cell::get);
        TAIL_DROPPED.with(|d| d.set(dropped_before));
        c.set(prev_cap);
        (r, dropped)
    })
}
