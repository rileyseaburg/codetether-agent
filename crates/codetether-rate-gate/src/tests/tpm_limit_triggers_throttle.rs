use crate::{RATE_GATE, record_usage};

#[test]
fn tpm_limit_triggers_throttle() {
    // bedrock/fable-tpm-test: 50 RPM, 100_000 TPM
    // Send 1 request with 99_999 tokens, then try to add 2_000 more
    record_usage("bedrock", "fable-tpm-test", 99_999);
    let cooldown = RATE_GATE.try_acquire("bedrock", "fable-tpm-test", 2_000);
    assert!(
        cooldown.is_some(),
        "should be throttled when TPM would be exceeded"
    );
}
