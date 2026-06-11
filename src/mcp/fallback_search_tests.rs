use serde_json::json;

#[test]
fn native_search_files_matches_filename_glob() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("one.rs"), "fn main() {}\n").expect("write");
    std::fs::write(dir.path().join("two.txt"), "hello\n").expect("write");
    let output = super::fallback_search_files::run(&json!({
        "path": dir.path().display().to_string(),
        "pattern": "*.rs"
    }))
    .expect("search");
    assert!(output.contains("one.rs"));
    assert!(!output.contains("two.txt"));
}

#[test]
fn native_grep_search_matches_contents() {
    let dir = tempfile::tempdir().expect("tempdir");
    let file = dir.path().join("note.txt");
    std::fs::write(&file, "Alpha\nBeta\n").expect("write");
    let output = super::fallback_grep::run(&json!({
        "path": dir.path().display().to_string(),
        "query": "alpha",
        "case_sensitive": false
    }))
    .expect("grep");
    assert!(output.contains("note.txt:1:Alpha"));
    assert!(!output.contains("Beta"));
}
