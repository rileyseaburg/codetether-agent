//! Tests for bounded proactive hierarchical range planning.

use super::ranges::planned;
use crate::session::index::Granularity;

#[test]
fn retains_active_tail_and_prepares_hierarchy() {
    let plans = planned(80);
    assert!(plans.iter().all(|plan| plan.range.end <= 72));
    assert!(
        plans
            .iter()
            .any(|plan| plan.range.start == 0 && plan.range.end == 64)
    );
    assert!(
        plans
            .iter()
            .any(|plan| plan.granularity == Granularity::Turn)
    );
    assert!(
        plans
            .iter()
            .any(|plan| plan.granularity == Granularity::Session)
    );
}

#[test]
fn long_sessions_keep_plans_bounded() {
    assert!(planned(10_000).len() <= 97);
}

#[test]
fn short_sessions_do_not_schedule_noise() {
    assert!(planned(11).is_empty());
}
