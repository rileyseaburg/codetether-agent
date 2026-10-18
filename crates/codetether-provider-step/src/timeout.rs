//! Non-streaming completion deadline configuration.
const DEFAULT_STEP_TIMEOUT_SECS: u64 = 300;

pub(super) fn step_timeout() -> std::time::Duration {
    let secs = std::env::var("CODETETHER_STEP_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_STEP_TIMEOUT_SECS);
    std::time::Duration::from_secs(secs)
}
