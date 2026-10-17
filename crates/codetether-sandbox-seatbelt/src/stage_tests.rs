use super::write;

#[test]
fn staged_profile_is_readable_and_unique() {
    let dir = tempfile::tempdir().expect("temp dir");
    let first = write("(version 1)\n(deny default)", dir.path()).expect("first profile");
    let second = write("(version 1)\n(deny default)", dir.path()).expect("second profile");
    assert_ne!(first, second, "each run needs its own profile file");
    assert_eq!(
        std::fs::read_to_string(&first).expect("read profile"),
        "(version 1)\n(deny default)"
    );
}

#[test]
fn staging_fails_when_directory_is_missing() {
    let dir = tempfile::tempdir().expect("temp dir");
    let missing = dir.path().join("absent");
    assert!(write("(version 1)", &missing).is_err());
}
