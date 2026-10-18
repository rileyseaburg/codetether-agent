//! Paste buffer regression retained from native input handling.
use super::fixture::paste;
use crate::*;

#[test]
fn placeholder_single_line_still_reports_one_line() {
    let p = paste(1, "single line");
    assert_eq!(p.lines(), 1);
    assert!(p.placeholder().contains("1 lines"));
}
