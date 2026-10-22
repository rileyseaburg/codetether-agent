use super::select;
use crate::summary_index::{Granularity, SummaryIndex, SummaryNode, SummaryRange};
fn index() -> SummaryIndex {
    let mut index = SummaryIndex::new();
    index.insert(
        SummaryRange::new(0, 16).unwrap(),
        SummaryNode {
            content: "Use client::read(id) observed in turn 2".into(),
            target_tokens: 10,
            granularity: Granularity::Phase,
            generation: 1,
            provenance: None,
        },
    );
    index
}
#[test]
fn summary_crossing_recent_boundary_preserves_earlier_evidence() {
    let index = index();
    let selected = select(&index, &[(0, 8)]);
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].range, SummaryRange::new(0, 16).unwrap());
    assert!(selected[0].content.contains("client::read(id)"));
    let mut keep = vec![false; 16];
    keep[8..].fill(true);
    let accepted =
        crate::selection::pack::select(&index, &mut keep, &[100; 16], (0..8).collect(), 10, |_| 10);
    assert_eq!(accepted.len(), 1);
    assert_eq!(keep, [vec![false; 8], vec![true; 8]].concat());
}
#[test]
fn one_summary_crossing_multiple_pins_is_not_duplicated() {
    assert_eq!(select(&index(), &[(0, 4), (6, 8), (10, 12)]).len(), 1);
    assert!(select(&index(), &[]).is_empty());
}
#[test]
fn already_retained_raw_history_does_not_subsidize_summary_cost() {
    let mut keep = vec![true; 16];
    keep[0] = false;
    let selected =
        crate::selection::pack::select(&index(), &mut keep, &[1; 16], vec![], 10, |_| 10);
    assert!(selected.is_empty());
}
