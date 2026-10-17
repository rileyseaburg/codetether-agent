//! Validate HTTP status and preserve Retry-After metadata before streaming.
use anyhow::Result;
mod error;
pub use error::format_openai_api_error;
mod validate;
pub use validate::validate_http_response;
const RETRY_AFTER_MARKER: &str = "retry_after_seconds=";

fn retry_after_seconds(headers: &reqwest::header::HeaderMap) -> Option<u64> {
    headers
        .get(reqwest::header::RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim()
        .parse()
        .ok()
}

fn annotate_retry_after(message: String, seconds: Option<u64>) -> String {
    match seconds {
        Some(value) => format!("{message} [{RETRY_AFTER_MARKER}{value}]"),
        None => message,
    }
}
#[cfg(test)]
mod tests;
