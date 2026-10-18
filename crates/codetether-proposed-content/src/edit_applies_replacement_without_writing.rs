//! Existing proposed-content regression case.
use crate::contents;
use crate::test_support::fixture;
use serde_json::json;

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
