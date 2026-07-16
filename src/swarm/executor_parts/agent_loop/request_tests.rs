use super::remaining;
use std::time::{Duration, Instant};

#[test]
fn configured_deadline_is_not_reduced_to_two_minutes() {
    let deadline = Instant::now() + Duration::from_secs(300);
    assert!(remaining(deadline, Instant::now()) > Duration::from_secs(240));
}
