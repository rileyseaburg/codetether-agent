//! Tests for the rate gate sliding-window logic.

#[cfg(test)]
mod tests {
    use crate::session::rate_gate::{RATE_GATE, check_rate_gate, record_usage};

    #[test]
    fn unlimited_provider_never_throttled() {
        // "unknown" provider has no limits → never throttled
        let result = check_rate_gate("unknown_provider", "some-model");
        assert!(
            result.is_none(),
            "unlimited provider should not be throttled"
        );
    }

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
}
