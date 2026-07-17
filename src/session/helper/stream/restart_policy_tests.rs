//! Unit tests for stream-restart backoff policy.

use std::time::Duration;

use super::restart::RestartPolicy;

#[test]
fn backoff_grows_exponentially() {
    let policy = RestartPolicy::default();
    assert_eq!(policy.backoff(0), Duration::from_secs(2));
    assert_eq!(policy.backoff(1), Duration::from_secs(4));
    assert_eq!(policy.backoff(2), Duration::from_secs(8));
}
