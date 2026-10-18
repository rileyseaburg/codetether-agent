//! Tests for durable-session workspace resolution.

use super::enclosing_workspace;
use std::path::{Path, PathBuf};

#[test]
fn derives_workspace_from_snapshot_layout() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sessions = dir.path().join(".codetether-agent/sessions");
    std::fs::create_dir_all(&sessions).expect("create sessions dir");
    let snapshot = sessions.join("abc-123.json");
    std::fs::write(&snapshot, "{}").expect("write snapshot");

    let resolved = enclosing_workspace(&snapshot).expect("workspace resolves");

    assert_eq!(
        resolved.canonicalize().expect("canonicalize resolved"),
        dir.path().canonicalize().expect("canonicalize workspace"),
    );
}

#[test]
fn rejects_paths_without_an_enclosing_workspace() {
    assert_eq!(enclosing_workspace(Path::new("/abc-123.json")), None);
}

#[test]
fn rejects_a_workspace_that_is_not_a_directory() {
    let missing = PathBuf::from("/nonexistent-root/.codetether-agent/sessions/abc-123.json");
    assert_eq!(enclosing_workspace(&missing), None);
}
