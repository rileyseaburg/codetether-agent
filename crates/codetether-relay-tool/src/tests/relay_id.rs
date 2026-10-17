//! Relay regression coverage.
use super::*;

pub(super) fn relay_id(prefix: &str) -> String {
    format!("{prefix}-{}", Uuid::new_v4().simple())
}
