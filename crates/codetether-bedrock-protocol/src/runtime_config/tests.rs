//! Existing runtime_config regression coverage.

use super::*;
#[test]
fn overrides_round_trip_without_env_mutation() {
    // These setters mutate a process-global cell shared with
    // `body::fields::tests`, so restore the prior values or those tests
    // observe this override and fail depending on execution order.
    let previous_effort = thinking_effort();
    let previous_tier = service_tier();
    set_thinking_effort(Some("high".to_string()));
    assert_eq!(thinking_effort().as_deref(), Some("high"));
    set_service_tier(None);
    assert_eq!(service_tier(), None);
    set_thinking_effort(previous_effort);
    set_service_tier(previous_tier);
}
