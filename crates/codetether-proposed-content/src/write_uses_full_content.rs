//! Existing proposed-content regression case.
use crate::contents;
use crate::test_support::fixture;
use serde_json::json;

#[test]
fn write_uses_full_content() {
    let (dir, path) = fixture();
    let args = json!({"path": path, "content": "fn fresh() {}\n"});
    let files = contents(dir.path(), "write", &args).unwrap();
    assert_eq!(files, vec![(path, "fn fresh() {}\n".into())]);
}
