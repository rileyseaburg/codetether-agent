//! Existing proposed-content regression case.
use crate::contents;
use crate::test_support::fixture;
use serde_json::json;

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
