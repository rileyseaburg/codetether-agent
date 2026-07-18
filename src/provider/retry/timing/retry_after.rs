//! Provider-suggested retry delay extraction from streamed error messages.

use regex::Regex;
use std::sync::OnceLock;
use std::time::Duration;

/// Parse Codex-compatible “try again in” seconds or milliseconds guidance.
pub(crate) fn from_message(message: &str) -> Option<Duration> {
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
