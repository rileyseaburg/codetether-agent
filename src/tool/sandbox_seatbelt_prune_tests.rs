use super::{MAX_AGE, stale};
use std::time::{Duration, SystemTime};

fn touch(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, "(version 1)").expect("write file");
    path
}

#[test]
fn removes_only_expired_profile_files() {
    let dir = tempfile::tempdir().expect("temp dir");
    let old = touch(dir.path(), "codetether-seatbelt-old.sb");
    let unrelated = touch(dir.path(), "important.txt");
    let future = SystemTime::now() + MAX_AGE + Duration::from_secs(60);

    stale(dir.path(), future);

    assert!(!old.exists(), "expired profile must be removed");
    assert!(unrelated.exists(), "unrelated files must be preserved");
}

#[test]
fn keeps_fresh_profiles_that_may_still_be_in_use() {
    let dir = tempfile::tempdir().expect("temp dir");
    let fresh = touch(dir.path(), "codetether-seatbelt-fresh.sb");

    stale(dir.path(), SystemTime::now());

    assert!(fresh.exists());
}

#[test]
fn missing_directory_is_not_an_error() {
    let dir = tempfile::tempdir().expect("temp dir");
    stale(&dir.path().join("absent"), SystemTime::now());
}