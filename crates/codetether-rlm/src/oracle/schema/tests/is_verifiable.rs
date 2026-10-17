//! Regression for is_verifiable.

use super::*;

#[test]
fn is_verifiable() {
    let grep_json = r#"{"kind": "grep", "file": "x.rs", "pattern": "fn", "matches": []}"#;
    let semantic_json = r#"{"kind": "semantic", "file": "x.rs", "answer": "text"}"#;

    assert!(FinalPayload::parse(grep_json).is_verifiable());
    assert!(!FinalPayload::parse(semantic_json).is_verifiable());
}
