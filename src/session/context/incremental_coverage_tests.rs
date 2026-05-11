//! Tests for [`super::incremental_coverage::uncovered_ranges`].

use crate::session::index::SummaryRange;

use super::incremental_coverage::uncovered_ranges;
use super::incremental_types::MessageOrigin;

#[test]
fn full_coverage_returns_empty() {
    let range = SummaryRange::new(0, 4).unwrap();
    let origins = vec![
        MessageOrigin::Summary(range),
        MessageOrigin::Clone(4),
        MessageOrigin::Clone(5),
    ];
    assert!(uncovered_ranges(&origins, 6).is_empty());
}

#[test]
fn isolates_dropped_gaps() {
    let summary = SummaryRange::new(2, 4).unwrap();
    let origins = vec![
        MessageOrigin::Clone(0),
        MessageOrigin::Clone(1),
        MessageOrigin::Summary(summary),
        MessageOrigin::Clone(4),
        MessageOrigin::Clone(6),
    ];
    assert_eq!(uncovered_ranges(&origins, 7), vec![(5, 6)]);
}

#[test]
fn treats_synthetic_as_invisible() {
    let origins = vec![
        MessageOrigin::Clone(0),
        MessageOrigin::Synthetic,
        MessageOrigin::Clone(3),
    ];
    assert_eq!(uncovered_ranges(&origins, 4), vec![(1, 3)]);
}
