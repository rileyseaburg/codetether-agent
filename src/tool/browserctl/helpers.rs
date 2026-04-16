//! Input-extraction helpers: base URL, token, and required/optional field accessors.

use super::input::BrowserCtlInput;
use anyhow::Result;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:4477";

pub(super) fn base_url(input: &BrowserCtlInput) -> String {
    input
        .base_url
        .clone()
        .or_else(|| std::env::var("BROWSERCTL_BASE").ok())
        .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
        .trim_end_matches('/')
        .to_string()
}

pub(super) fn token(input: &BrowserCtlInput) -> Option<String> {
    input
        .token
        .clone()
        .or_else(|| std::env::var("BROWSERCTL_TOKEN").ok())
        .filter(|value| !value.trim().is_empty())
}

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
