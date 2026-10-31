//! Session IDs and symlink aliases cannot multiply writers for one database.
#[test]
fn session_locators_share_their_database_writer() {
    let root = tempfile::tempdir().unwrap();
    let first = super::get(&root.path().join("first.json")).unwrap();
    let second = super::get(&root.path().join("second.json")).unwrap();
    assert!(std::sync::Arc::ptr_eq(&first, &second));
}

#[cfg(unix)]
#[test]
fn symlinked_directories_share_their_database_writer() {
    let root = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(root.path(), root.path().join("alias")).unwrap();
    let first = super::get(&root.path().join("first.json")).unwrap();
    let second = super::get(&root.path().join("alias/second.json")).unwrap();
    assert!(std::sync::Arc::ptr_eq(&first, &second));
}
