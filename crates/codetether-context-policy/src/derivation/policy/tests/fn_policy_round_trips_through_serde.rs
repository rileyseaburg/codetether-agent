//! policy round trips through serde derivation policy.
use super::*;

#[test]
pub(super) fn policy_round_trips_through_serde() {
    let p = DerivePolicy::Reset {
        threshold_tokens: 12_000,
    };
    let json = serde_json::to_string(&p).unwrap();
    assert!(json.contains("\"policy\":\"reset\""));
    let back: DerivePolicy = serde_json::from_str(&json).unwrap();
    assert!(matches!(
        back,
        DerivePolicy::Reset {
            threshold_tokens: 12_000
        }
    ));
}
