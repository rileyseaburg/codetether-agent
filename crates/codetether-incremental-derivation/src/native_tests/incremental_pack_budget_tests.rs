//! Regressions for charging already-retained messages and oversized summaries.
use super::{fixtures, select};

#[test]
fn protected_messages_are_not_charged_twice() {
    let mut keep = [true, false];
    let gaps = select(&Default::default(), &mut keep, &[5, 10], vec![0, 1], 10);
    assert!(gaps.is_empty());
    assert_eq!(keep, [true, true]);
}

#[test]
fn summary_must_fit_and_be_smaller_than_its_raw_range() {
    let (index, cost) = fixtures::prepared();
    let mut keep = [false, false];
    assert!(select(&index, &mut keep, &[1000; 2], vec![0, 1], cost - 1).is_empty());
    assert_eq!(keep, [false, false]);
    assert!(select(&index, &mut keep, &[1; 2], vec![0, 1], cost).is_empty());
    assert_eq!(keep, [true, true]);
}

#[test]
fn score_order_changes_do_not_rewrite_the_prepared_prefix() {
    let (index, cost) = fixtures::prepared();
    let a = select(&index, &mut [false; 3], &[1000; 3], vec![0, 1, 2], cost);
    let b = select(&index, &mut [false; 3], &[1000; 3], vec![2, 1, 0], cost);
    assert_eq!(a.len(), 1);
    assert_eq!(a[0].range, b[0].range);
    assert_eq!(a[0].content, b[0].content);
}
