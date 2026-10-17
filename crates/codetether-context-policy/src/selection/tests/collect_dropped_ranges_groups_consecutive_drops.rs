//! Existing context selection regression.
use super::*;

#[test]
fn collect_dropped_ranges_groups_consecutive_drops() {
    let keep = vec![true, false, false, true, false, true];
    assert_eq!(collect_dropped_ranges(&keep), vec![(1, 3), (4, 5)]);
}
