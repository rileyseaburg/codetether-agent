const DEFAULT_WINDOW: usize = 1_000;
const MAX_WINDOW: usize = 10_000;

pub fn session_resume_window() -> usize {
    let parsed = std::env::var("CODETETHER_SESSION_RESUME_WINDOW")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0);
    match parsed {
        Some(value) if value > MAX_WINDOW => {
            tracing::warn!(
                requested = value,
                clamped = MAX_WINDOW,
                "session resume window too large; clamping"
            );
            MAX_WINDOW
        }
        Some(value) => value,
        None => DEFAULT_WINDOW,
    }
}
