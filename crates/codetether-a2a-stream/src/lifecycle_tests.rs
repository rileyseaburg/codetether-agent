//! Regression cases for the parent stream module.

use super::{LifecycleState, next_state};

#[test]
fn success_goes_live() {
    assert_eq!(
        next_state(LifecycleState::Backoff, true, false),
        LifecycleState::Live
    );
}

#[test]
fn failure_backs_off_until_breaker_opens() {
    assert_eq!(
        next_state(LifecycleState::Live, false, false),
        LifecycleState::Backoff
    );
    assert_eq!(
        next_state(LifecycleState::Backoff, false, true),
        LifecycleState::Dead
    );
}
