//! Input-extraction helpers for required and optional browserctl fields.

use anyhow::Result;

pub(super) fn require_string<'a>(value: &'a Option<String>, field: &str) -> Result<&'a str> {
    value
        .as_deref()
        .filter(|v| !v.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("{field} is required for this browserctl action"))
}

pub(super) fn require_index(value: Option<usize>, field: &str) -> Result<usize> {
    value.ok_or_else(|| anyhow::anyhow!("{field} is required for this browserctl action"))
}

pub(super) fn require_point(value: Option<f64>, field: &str) -> Result<f64> {
    value.ok_or_else(|| anyhow::anyhow!("{field} is required for this browserctl action"))
}

pub(super) fn optional_string(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|v| !v.trim().is_empty())
}
