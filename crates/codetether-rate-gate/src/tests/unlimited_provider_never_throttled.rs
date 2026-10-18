use crate::check_rate_gate;

#[test]
fn unlimited_provider_never_throttled() {
    // "unknown" provider has no limits → never throttled
    let result = check_rate_gate("unknown_provider", "some-model");
    assert!(
        result.is_none(),
        "unlimited provider should not be throttled"
    );
}
