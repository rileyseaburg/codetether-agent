//! Shared resident accounting and LRU ordering.
pub(crate) use codetether_collaboration_state::residency::{forget, touch};
pub(super) use codetether_collaboration_state::residency::{pop_candidate, resident_count, usage};
