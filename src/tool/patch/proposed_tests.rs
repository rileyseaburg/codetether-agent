#[test]
fn reconstructs_proposed_content_without_writing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("src/lib.rs");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "fn old() {}\n").unwrap();
    let patch = "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1,1 +1,1 @@\n-fn old() {}\n+fn new() {}\n";

    let files = super::contents(dir.path(), patch).unwrap();

    assert_eq!(files, vec![(path.clone(), "fn new() {}".into())]);
    assert_eq!(std::fs::read_to_string(path).unwrap(), "fn old() {}\n");
}
