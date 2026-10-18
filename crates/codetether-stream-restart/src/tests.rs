//! Original backoff and provider-hint policy regression tests.
use crate::RestartPolicy;
use std::time::Duration;

#[test]
fn backoff_grows_exponentially() {
    let policy = RestartPolicy::default();
    for (attempt, nominal_ms) in [(0, 200), (1, 400), (2, 800)] {
        let delay = policy.backoff(attempt);
        assert!(delay >= Duration::from_millis(nominal_ms * 9 / 10));
        assert!(delay <= Duration::from_millis(nominal_ms * 11 / 10));
    }
}

#[test]
fn provider_retry_guidance_overrides_backoff() {
    let stop = codetether_stream_outcome::StreamStop::Fault {
        transient: true,
        message: "Rate limit reached. Try again in 1.898s.".into(),
    };
    assert_eq!(
        RestartPolicy::default().delay(0, &stop),
        Duration::from_secs_f64(1.898)
    );
}
