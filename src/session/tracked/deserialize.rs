//! Preserve the legacy bounded-deserializer contract during migration.
use super::TrackedVec;
use serde::{Deserialize, Deserializer};
pub(crate) fn tail<'de, D, T>(deserializer: D) -> Result<TrackedVec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    crate::session::tail_seed::deserialize_tail_vec(deserializer).map(Into::into)
}
