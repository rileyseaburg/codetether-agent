use super::decision::{LeaseDecision, LeaseState, decide, owns};
use chrono::{Duration, Utc};

fn state(holder: &str, age_seconds: i64) -> LeaseState {
    LeaseState {
        holder: Some(holder.into()),
        renewed_at: Some(Utc::now() - Duration::seconds(age_seconds)),
        duration_seconds: 30,
    }
}

#[test]
fn missing_lease_is_acquired() {
    assert_eq!(decide(None, "mine", Utc::now()), LeaseDecision::Acquire);
}

#[test]
fn same_holder_renews() {
    assert_eq!(
        decide(Some(&state("mine", 0)), "mine", Utc::now()),
        LeaseDecision::Renew
    );
}

#[test]
fn active_owner_conflicts() {
    assert!(matches!(
        decide(Some(&state("other", 0)), "mine", Utc::now()),
        LeaseDecision::Conflict { holder, .. } if holder == "other"
    ));
}

#[test]
fn expired_owner_is_replaced() {
    assert_eq!(
        decide(Some(&state("other", 31)), "mine", Utc::now()),
        LeaseDecision::TakeOver
    );
}

#[test]
fn stale_owner_cannot_release_successor() {
    assert!(owns(&state("successor", 0), "successor"));
    assert!(!owns(&state("successor", 0), "stale-owner"));
}
