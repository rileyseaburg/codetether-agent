//! Extraction of provider-requested retry delays from errors.

const MARKER: &str = "retry_after_seconds=";
const MAX_SECONDS: u64 = 60;

pub(super) fn seconds(error: &anyhow::Error) -> Option<u64> {
    let message = format!("{error:#}");
    let value = message
        .split(MARKER)
        .nth(1)?
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse::<u64>()
        .ok()?;
    Some(value.clamp(1, MAX_SECONDS))
}

#[cfg(test)]
#[path = "retry_after_tests.rs"]
mod tests;
