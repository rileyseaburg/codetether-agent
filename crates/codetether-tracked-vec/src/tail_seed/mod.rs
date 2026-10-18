//! Bounded-memory sequence deserialization with per-thread cap accounting.
//!
//! [with_tail_cap] selects how many newest entries a [deserialize_tail_vec]
//! call retains and reports the total discarded across all such sequences.
mod scope;
mod state;
mod visitor;
pub use scope::with_tail_cap;
use serde::de::{Deserialize, Deserializer};
use std::marker::PhantomData;
/// Deserialize the newest entries under the active thread-local cap.
/// # Arguments
/// `d` is a Serde sequence deserializer; call [with_tail_cap] to select a cap.
/// # Returns
/// The retained suffix, or the entire sequence when no cap is active.
/// # Errors
/// Propagates Serde errors for invalid sequences or element values.
/// # Examples
/// This function can be used with Serde's `deserialize_with` field attribute.
pub fn deserialize_tail_vec<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    d.deserialize_seq(visitor::V::<T>(PhantomData))
}
