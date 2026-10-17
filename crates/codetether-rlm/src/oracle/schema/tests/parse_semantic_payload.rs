//! Regression for parse_semantic_payload.

use super::*;

#[test]
fn parse_semantic_payload() {
    let json = r#"{
            "kind": "semantic",
            "file": "src/main.rs",
            "answer": "This module provides async processing."
        }"#;

    let payload = FinalPayload::parse(json);
    match payload {
        FinalPayload::Semantic(p) => {
            assert_eq!(p.file, "src/main.rs");
            assert!(p.answer.contains("async processing"));
        }
        _ => panic!("Expected Semantic payload"),
    }
}
