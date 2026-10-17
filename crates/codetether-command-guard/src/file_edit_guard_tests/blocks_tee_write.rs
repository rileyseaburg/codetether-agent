//! File-edit guard regression.
use super::*;

#[test]
fn blocks_tee_write() {
    assert!(file_edit_guard_reason("echo hi | tee out.txt").is_some());
}
