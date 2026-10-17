//! PR body display.
use super::*;
pub(super) fn display(value: Option<&str>) -> &str {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("unknown")
}
