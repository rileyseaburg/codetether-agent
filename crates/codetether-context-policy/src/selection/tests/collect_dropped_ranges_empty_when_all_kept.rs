//! Existing context selection regression.
use super::*;

#[test]
fn collect_dropped_ranges_empty_when_all_kept() {
    let keep = vec![true, true, true];
    assert_eq!(collect_dropped_ranges(&keep), Vec::<(usize, usize)>::new());
}
