//! Shared forage regression coverage.
use super::*;

#[test]
fn urgency_bonus_increases_for_due_dates() {
    let now = Utc::now();
    let overdue = urgency_bonus(Some(now - Duration::days(1)), now);
    let soon = urgency_bonus(Some(now + Duration::days(3)), now);
    let later = urgency_bonus(Some(now + Duration::days(45)), now);
    assert!(overdue > soon);
    assert!(soon > later);
}
