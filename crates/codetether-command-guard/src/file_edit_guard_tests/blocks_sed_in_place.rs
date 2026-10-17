//! File-edit guard regression.
use super::*;

#[test]
fn blocks_sed_in_place() {
    assert!(file_edit_guard_reason("sed -i 's/a/b/' file.rs").is_some());
}
