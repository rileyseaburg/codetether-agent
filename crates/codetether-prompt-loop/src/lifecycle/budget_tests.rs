use super::exhausted;

#[test]
fn unset_budget_allows_long_sessions() {
    assert!(!exhausted(usize::MAX, None));
}

#[test]
fn explicit_budget_still_stops_at_its_limit() {
    assert!(!exhausted(49, Some(50)));
    assert!(exhausted(50, Some(50)));
}
