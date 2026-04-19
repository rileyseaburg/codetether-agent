//! Bounded-memory sequence deserialization via a thread-local tail cap.
//!
//! When [`with_tail_cap`] is active, any [`Vec`] field annotated with
//! `#[serde(deserialize_with = "deserialize_tail_vec")]` keeps only the
//! last N elements of the JSON array, discarding older ones as it streams
//! through the input. This lets the TUI resume a multi-megabyte session
//! file without holding every historical message in memory.

use std::cell::Cell;
use std::collections::VecDeque;
use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};

thread_local! {
    static TAIL_CAP: Cell<usize> = const { Cell::new(usize::MAX) };
    static TAIL_DROPPED: Cell<usize> = const { Cell::new(0) };
}

/// Run `f` with the tail cap set to `cap` for the current thread. Returns
/// `(result, dropped)` where `dropped` is the total number of elements
/// discarded across all tail-capped sequences during the call.
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

/// Serde `deserialize_with` helper that keeps only the last N elements.
pub fn deserialize_tail_vec<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct V<T>(PhantomData<T>);
    impl<'de, T: Deserialize<'de>> Visitor<'de> for V<T> {
        type Value = Vec<T>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a JSON array")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Vec<T>, A::Error> {
            let cap = TAIL_CAP.with(Cell::get);
            // Pre-reserve the ring buffer at exactly `cap` capacity so
            // steady-state deserialization never grows/reallocates. For
            // the common case of a large session clamped to
            // `SESSION_RESUME_WINDOW = 200`, this pre-allocates ~200
            // slots once instead of growing the VecDeque ~8 times.
            let initial = cap.min(a.size_hint().unwrap_or(0)).min(1 << 16);
            let mut buf: VecDeque<T> = VecDeque::with_capacity(initial);
            let mut dropped: usize = 0;
            while let Some(v) = a.next_element::<T>()? {
                if cap == 0 {
                    dropped += 1;
                    continue;
                }
                if buf.len() == cap {
                    buf.pop_front();
                    dropped += 1;
                }
                buf.push_back(v);
            }
            if dropped > 0 {
                TAIL_DROPPED.with(|d| d.set(d.get().saturating_add(dropped)));
            }
            // `VecDeque::into` is O(1) when head == 0; once we've hit the
            // cap and popped at least once, head > 0 and the conversion
            // shifts. `make_contiguous()` then `into()` guarantees the
            // single shift happens here, not hidden in a later access.
            buf.make_contiguous();
            Ok(buf.into())
        }
    }
    d.deserialize_seq(V::<T>(PhantomData))
}
