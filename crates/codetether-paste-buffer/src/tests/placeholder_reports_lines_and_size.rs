//! Paste buffer regression retained from native input handling.
use super::fixture::paste;
use crate::*;

#[test]
fn placeholder_reports_lines_and_size() {
    let p = paste(7, "a\nb\nc");
    assert_eq!(p.placeholder(), "[Pasted text #7: 3 lines, 5 B]");
}
