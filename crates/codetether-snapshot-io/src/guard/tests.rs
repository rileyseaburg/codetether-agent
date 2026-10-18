//! A cached hash is not proof of the current on-disk snapshot.

use super::{is_unchanged, record_saved};

#[test]
fn external_replacement_invalidates_save_elision() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.json");
    let id = uuid::Uuid::new_v4().to_string();
    std::fs::write(&path, b"original").unwrap();
    record_saved(&id, b"original");
    assert!(is_unchanged(&id, b"original", &path));
    std::fs::write(&path, b"external").unwrap();
    assert!(!is_unchanged(&id, b"original", &path));
}

#[test]
fn same_id_in_different_directory_does_not_elide() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.json");
    let id = uuid::Uuid::new_v4().to_string();
    record_saved(&id, b"other-directory");
    std::fs::write(&path, b"this-directory").unwrap();
    assert!(!is_unchanged(&id, b"other-directory", &path));
    assert!(!is_unchanged(
        &id,
        b"other-directory",
        &dir.path().join("missing")
    ));
}
