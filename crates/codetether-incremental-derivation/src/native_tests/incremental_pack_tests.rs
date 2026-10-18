//! Budget and prefix-stability regressions for prepared-summary packing.
use super::select;
#[path = "incremental_pack_budget_tests.rs"]
mod budget;
#[path = "incremental_pack_fixtures.rs"]
mod fixtures;
#[path = "incremental_pack_projection_tests.rs"]
mod projection;

#[test]
fn reserves_summary_tokens_before_raw_messages_consume_the_budget() {
    let (index, cost) = fixtures::prepared();
    let mut keep = [false, false, false, true];
    let costs = [1000, 1000, cost, 1];
    let gaps = select(&index, &mut keep, &costs, vec![2, 1, 0], cost);
    assert_eq!(gaps.len(), 1);
    assert_eq!(keep, [false, false, false, true]);
}

#[test]
fn summary_coverage_is_not_duplicated_as_raw_history() {
    let (index, cost) = fixtures::prepared();
    let mut keep = [false, false, false, true];
    let gaps = select(
        &index,
        &mut keep,
        &[1000, 1000, 10, 1],
        vec![0, 1, 2],
        cost + 2010,
    );
    assert_eq!(gaps.len(), 1);
    assert_eq!(keep, [false, false, true, true]);
}

#[test]
fn summary_cache_miss_falls_back_to_raw_packing_without_io() {
    let mut keep = [false, false];
    let gaps = select(&Default::default(), &mut keep, &[10, 10], vec![1, 0], 10);
    assert!(gaps.is_empty());
    assert_eq!(keep, [false, true]);
}

#[test]
fn preserves_pins_and_skips_summaries_that_overlap_them() {
    let (index, _) = fixtures::prepared();
    let mut keep = [true, false, true];
    let gaps = select(&index, &mut keep, &[1000, 1000, 1], vec![1, 0], 100);
    assert!(gaps.is_empty());
    assert_eq!(keep, [true, false, true]);
}
