//! Regression for file_extraction.

use super::*;

#[test]
fn file_extraction() {
    let grep_json = r#"{"kind": "grep", "file": "src/main.rs", "pattern": "fn", "matches": []}"#;
    assert_eq!(FinalPayload::parse(grep_json).file(), Some("src/main.rs"));
}
