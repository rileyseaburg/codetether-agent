//! OpenAI Codex overload detection and cooldown policy.

use std::time::Duration;

const DEFAULT_COOLDOWN_SECS: u64 = 30;
const MAX_COOLDOWN_SECS: u64 = 300;
const RETRY_AFTER: &str = "retry_after_seconds=";

pub fn codex_overload(error: &str) -> bool {
    let lower = error.to_ascii_lowercase();
    lower.contains("servers are currently overloaded")
        || lower.contains("server_is_overloaded")
        || lower.contains("codex-retryable: overloaded")
}

pub fn codex_overload_cooldown(error: &str) -> Duration {
    let encoded = error.split(RETRY_AFTER).nth(1).and_then(|tail| {
        tail.chars()
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .ok()
    });
    let configured = std::env::var("CODETETHER_CODEX_OVERLOAD_COOLDOWN_SECS")
        .ok()
        .and_then(|value| value.parse().ok());
    Duration::from_secs(
        encoded
            .or(configured)
            .unwrap_or(DEFAULT_COOLDOWN_SECS)
            .clamp(5, MAX_COOLDOWN_SECS),
    )
}
