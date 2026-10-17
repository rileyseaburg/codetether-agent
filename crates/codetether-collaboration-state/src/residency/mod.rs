//! Resident capacity accounting, LRU order and activation locks.
//!
//! [Slot] reserves pending capacity until committed or dropped. [touch] and
//! [pop_candidate] maintain the existing resident order, while [acquire] keeps
//! activation of one durable child serialized without blocking unrelated IDs.
mod order;
mod singleflight;
mod slot;
pub mod stale;
mod state;
pub use order::{forget, pop_candidate, resident_count, touch, usage};
pub use singleflight::acquire;
pub use slot::Slot;
#[cfg(test)]
mod singleflight_tests;
#[cfg(test)]
mod slot_tests;
