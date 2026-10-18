//! Preserve the legacy bounded-deserializer contract during migration.
use super::TrackedVec;
use serde::{Deserialize, Deserializer};
/// Deserialize a tracked vector using the active per-thread tail limit.
/// # Arguments
/// `deserializer` supplies the sequence to retain.
/// # Returns
/// A new tracked vector whose retained contents start dirty at zero.
/// # Errors
/// Propagates Serde sequence and element errors.
/// # Examples
/// ```rust
/// #[derive(serde::Deserialize)]
/// struct Record {
///     #[serde(deserialize_with = "codetether_tracked_vec::tail")]
///     values: codetether_tracked_vec::TrackedVec<u32>,
/// }
/// let record = Record { values: vec![1, 2].into() };
/// assert_eq!(record.values.as_slice(), &[1, 2]);
/// ```
pub fn tail<'de, D, T>(deserializer: D) -> Result<TrackedVec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    crate::tail_seed::deserialize_tail_vec(deserializer).map(Into::into)
}
