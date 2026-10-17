//! Store updates for new, repeated, contested and stale beliefs.
//!
//! [merge_beliefs] confirms duplicates and emits new-belief events.
//! [decay_stale_beliefs] creates attention for overdue active beliefs.
//! Functions mutate caller-owned collections and do not acquire locks.
mod confirm;
mod contest;
mod decay;
mod merge;
pub use confirm::{confirm_existing, existing_id};
pub use contest::apply_contradictions;
pub use decay::decay_stale_beliefs;
pub use merge::merge_beliefs;
