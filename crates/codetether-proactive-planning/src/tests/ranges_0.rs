use crate::planned;
use codetether_context_policy::summary_index::Granularity;

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
