//! Tests for forage goal-clarification helpers.

use super::{EmptyReason, classify_empty};

#[test]
fn classify_no_okrs() {
    assert_eq!(classify_empty(0), EmptyReason::NoOkrs);
}

#[test]
fn classify_with_okrs_but_no_work() {
    assert_eq!(classify_empty(3), EmptyReason::NoActionableWork);
}
