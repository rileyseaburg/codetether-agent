//! Every write-capable tool yields analyzable proposed content.

use serde_json::json;

use super::{contents, supported};

fn fixture() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("src/lib.rs");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "fn old() {}\nfn keep() {}\n").unwrap();
    (dir, path)
}

#[test]
fn write_uses_full_content() {
    let (dir, path) = fixture();
    let args = json!({"path": path, "content": "fn fresh() {}\n"});
    let files = contents(dir.path(), "write", &args).unwrap();
    assert_eq!(files, vec![(path, "fn fresh() {}\n".into())]);
}

#[test]
fn edit_applies_replacement_without_writing() {
    let (dir, path) = fixture();
    let args = json!({"path": path, "old_string": "fn old()", "new_string": "fn new()"});
    let files = contents(dir.path(), "edit", &args).unwrap();
    assert_eq!(files[0].1, "fn new() {}\nfn keep() {}\n");
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        "fn old() {}\nfn keep() {}\n"
    );
}

#[test]
fn multiedit_chains_edits_to_the_same_file() {
    let (dir, path) = fixture();
    let args = json!({"edits": [
        {"file": path, "old_string": "fn old()", "new_string": "fn a()"},
        {"file": path, "old_string": "fn keep()", "new_string": "fn b()"}
    ]});
    let files = contents(dir.path(), "multiedit", &args).unwrap();
    assert_eq!(files, vec![(path, "fn a() {}\nfn b() {}\n".into())]);
}

#[test]
fn unsupported_tools_are_rejected() {
    assert!(!supported("bash"));
    assert!(contents(std::path::Path::new("/"), "bash", &json!({})).is_err());
}
