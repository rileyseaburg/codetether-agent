//! Provider-suggested retry delay extraction from error messages.

use regex::Regex;
use std::sync::OnceLock;
use std::time::Duration;

/// Parse “try again in” seconds or milliseconds guidance.
///
/// # Arguments
///
/// * `message` — Provider error text with an optional delay hint.
///
/// # Returns
///
/// The suggested delay, or `None` when missing or unrepresentable.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::retry::timing::from_message;
/// use std::time::Duration;
/// assert_eq!(from_message("Please try again in 28ms."), Some(Duration::from_millis(28)));
/// ```
pub fn from_message(message: &str) -> Option<Duration> {
    let captures = matcher().captures(message)?;
    let value = captures.get(1)?.as_str().parse::<f64>().ok()?;
    let unit = captures.get(2)?.as_str().to_ascii_lowercase();
    if unit == "ms" {
        return finite_millis(value);
    }
    Duration::try_from_secs_f64(value).ok()
}

fn finite_millis(value: f64) -> Option<Duration> {
    value
        .is_finite()
        .then(|| Duration::from_millis(value as u64))
}

fn matcher() -> &'static Regex {
    static MATCHER: OnceLock<Regex> = OnceLock::new();
    MATCHER.get_or_init(|| {
        Regex::new(r"(?i)try again in\s*(\d+(?:\.\d+)?)\s*(s|ms|seconds?)")
            .expect("retry-after regex must compile")
    })
}
