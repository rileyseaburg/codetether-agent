use super::jittered;
use std::time::Duration;

#[test]
fn jitter_stays_within_codex_backoff_bands() {
    for (attempt, nominal_ms) in [(0, 200), (1, 400), (2, 800)] {
        for _ in 0..32 {
            let delay = jittered(Duration::from_millis(200), 2, attempt);
            assert!(delay >= Duration::from_millis(nominal_ms * 9 / 10));
            assert!(delay <= Duration::from_millis(nominal_ms * 11 / 10));
        }
    }
}

#[test]
fn zero_backoff_remains_zero() {
    assert_eq!(jittered(Duration::ZERO, 2, 8), Duration::ZERO);
}
