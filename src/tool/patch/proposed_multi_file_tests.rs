//! Exercise the reconstruction used by guardrail preapproval without writes.

#[test]
fn multi_file_preapproval_reconstruction_keeps_headers_out_of_content() {
    let root = tempfile::tempdir().unwrap();
    let one = root.path().join("one.rs");
    let two = root.path().join("two.rs");
    std::fs::write(&one, "old one\n").unwrap();
    std::fs::write(&two, "old two\n").unwrap();
    let patch = "--- a/one.rs\n+++ b/one.rs\n@@ -1 +1 @@\n-old one\n+new one\n\
                 --- a/two.rs\n+++ b/two.rs\n@@ -1 +1 @@\n-old two\n+new two\n\
                 --- /dev/null\n+++ b/three.rs\n@@ -0,0 +1 @@\n+created\n";
    let files = super::super::contents(root.path(), patch).unwrap();
    assert!(files.contains(&(one.clone(), "new one".into())));
    assert!(files.contains(&(two.clone(), "new two".into())));
    assert!(files.contains(&(root.path().join("three.rs"), "created".into())));
    assert_eq!(std::fs::read_to_string(one).unwrap(), "old one\n");
    assert_eq!(std::fs::read_to_string(two).unwrap(), "old two\n");
    assert!(!root.path().join("three.rs").exists());
}

#[test]
fn zero_context_preapproval_insertion_respects_declared_position() {
    let root = tempfile::tempdir().unwrap();
    let file = root.path().join("file.rs");
    std::fs::write(&file, "first\nsecond\n").unwrap();
    let patch = "--- a/file.rs\n+++ b/file.rs\n@@ -2,0 +3 @@\n+third\n";
    let files = super::super::contents(root.path(), patch).unwrap();
    assert_eq!(files, vec![(file.clone(), "first\nsecond\nthird".into())]);
    assert_eq!(std::fs::read_to_string(file).unwrap(), "first\nsecond\n");
}
