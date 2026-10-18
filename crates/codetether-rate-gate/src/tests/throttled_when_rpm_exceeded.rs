use crate::{RATE_GATE, record_usage};

#[test]
fn throttled_when_rpm_exceeded() {
    // Fill the window beyond 50 RPM
    for _ in 0..55 {
        record_usage("bedrock", "fable-rpm-test", 10);
    }
    // Should now be throttled
    let cooldown = RATE_GATE.try_acquire("bedrock", "fable-rpm-test", 10);
    assert!(cooldown.is_some(), "should be throttled after 55 requests");
    assert!(
        cooldown.unwrap().as_millis() >= 1_000,
        "cooldown should be at least 1 second"
    );
}
