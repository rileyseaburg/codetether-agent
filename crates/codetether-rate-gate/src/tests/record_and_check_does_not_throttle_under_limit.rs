use crate::{RATE_GATE, record_usage};

#[test]
fn record_and_check_does_not_throttle_under_limit() {
    // bedrock/fable limit: 50 RPM, 100_000 TPM
    // Record 10 small requests — well under limit
    for _ in 0..10 {
        record_usage("bedrock", "fable", 500);
    }
    let result = RATE_GATE.try_acquire("bedrock", "fable", 500);
    assert!(result.is_none(), "should have headroom after 10 requests");
}
