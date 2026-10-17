//! File-edit guard regression.
use super::*;

#[test]
fn blocks_cat_redirect() {
    assert!(file_edit_guard_reason("cat >> notes.txt").is_some());
}
