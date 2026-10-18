//! Stream a sequence through a bounded tail ring buffer.
use super::state::{TAIL_CAP, TAIL_DROPPED};
use serde::de::{Deserialize, SeqAccess, Visitor};
use std::{cell::Cell, collections::VecDeque, fmt, marker::PhantomData};

pub(super) struct V<T>(pub(super) PhantomData<T>);

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
