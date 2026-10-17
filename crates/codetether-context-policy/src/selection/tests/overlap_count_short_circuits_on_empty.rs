//! Existing context selection regression.
use super::*;

#[test]
fn overlap_count_short_circuits_on_empty() {
    let left: Vec<String> = vec!["a".into()];
    let right: Vec<String> = Vec::new();
    assert_eq!(overlap_count(&left, &right), 0);
}
