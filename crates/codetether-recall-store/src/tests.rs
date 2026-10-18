//! Deletion guard regression test moved from the native recall suite.

#[test]
fn deletion_tombstone_blocks_and_restore_releases() {
    let id = "recall-delete-race";
    crate::tombstone::mark(id);
    assert!(crate::tombstone::contains(id));
    crate::tombstone::restore(id);
    assert!(!crate::tombstone::contains(id));
}
