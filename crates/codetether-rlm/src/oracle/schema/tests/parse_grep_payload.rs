//! Regression for parse_grep_payload.

use super::*;

#[test]
fn parse_grep_payload() {
    let json = r#"{
            "kind": "grep",
            "file": "src/main.rs",
            "pattern": "async fn",
            "matches": [
                {"line": 42, "text": "async fn process() {"},
                {"line": 100, "text": "async fn handle() {"}
            ]
        }"#;

    let payload = FinalPayload::parse(json);
    match payload {
        FinalPayload::Grep(p) => {
            assert_eq!(p.file, "src/main.rs");
            assert_eq!(p.pattern, "async fn");
            assert_eq!(p.matches.len(), 2);
            assert_eq!(p.matches[0].line, 42);
        }
        _ => panic!("Expected Grep payload"),
    }
}
