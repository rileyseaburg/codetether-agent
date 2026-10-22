//! Mutation-aware vectors for incremental persistence and bounded loading.
//!
//! [TrackedVec] retains vector access and JSON serialization while tracking
//! dirty suffixes and replacement identities. [tail] cooperates with
//! [tail_seed::with_tail_cap] to deserialize only the newest sequence entries.
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, AtomicUsize};
mod access;
mod append;
mod conversion;
mod deserialize;
mod indexing;
mod iteration;
pub mod tail_seed;
mod tracking;
mod version;
pub use deserialize::tail;

/// A vector that records its earliest changed element.
/// Indexed element and range writes mark only their affected suffix. Mutations
/// through unrestricted mutable vector access conservatively mark the whole vector.
///
/// # Examples
/// ```
/// use codetether_tracked_vec::TrackedVec;
/// let mut values = TrackedVec::from(vec![1]);
/// values.push(2);
/// assert_eq!(values.as_slice(), &[1, 2]);
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrackedVec<T> {
    values: Vec<T>,
    #[serde(skip)]
    dirty: AtomicUsize,
    #[serde(skip, default = "version::new")]
    version: AtomicU64,
}
