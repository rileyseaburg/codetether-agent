use super::WorktreeManager;

#[test]
fn corruption_detection_matches_missing_blob() {
    let output = "error: missing blob 1234abcd";
    assert!(WorktreeManager::looks_like_object_corruption(output));
}

#[test]
fn corruption_detection_ignores_non_corruption_errors() {
    let output = "fatal: not a git repository";
    assert!(!WorktreeManager::looks_like_object_corruption(output));
}

#[test]
fn summarize_output_uses_first_non_empty_line() {
    let output = "\n\nfatal: bad object HEAD\nmore";
    assert_eq!(
        WorktreeManager::summarize_git_output(output),
        "fatal: bad object HEAD"
    );
}
