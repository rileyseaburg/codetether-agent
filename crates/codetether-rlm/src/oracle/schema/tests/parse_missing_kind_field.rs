//! Regression for parse_missing_kind_field.

use super::*;

#[test]
fn parse_missing_kind_field() {
    let json = r#"{"file": "src/main.rs", "data": "value"}"#;
    let payload = FinalPayload::parse(json);
    match payload {
        FinalPayload::Malformed { error, .. } => {
            assert!(error.contains("kind"));
        }
        _ => panic!("Expected Malformed payload"),
    }
}
