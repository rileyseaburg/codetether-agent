//! Workspace binding accepts canonical aliases but never foreign or missing paths.
use super::super::{http_error, scope::verify_paths};
use std::path::Path;

#[test]
fn accepts_canonical_aliases() {
    let dir = tempfile::tempdir().unwrap();
    verify_paths(dir.path(), &dir.path().join(".")).unwrap();
}

#[test]
fn rejects_foreign_missing_and_non_directory_paths_without_mutation() {
    let dir = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    let snapshot = dir.path().join("session.json");
    std::fs::write(&snapshot, "original durable state").unwrap();
    for recorded in [
        other.path(),
        &dir.path().join("missing"),
        snapshot.as_path(),
    ] {
        let error = verify_paths(recorded, dir.path()).unwrap_err();
        let (status, message) = http_error(error);
        assert_eq!(status.as_u16(), 403);
        assert!(!message.contains(dir.path().to_str().unwrap()));
        assert_eq!(
            std::fs::read_to_string(&snapshot).unwrap(),
            "original durable state"
        );
    }
    assert!(verify_paths(Path::new("/missing/session/workspace"), dir.path()).is_err());
}
