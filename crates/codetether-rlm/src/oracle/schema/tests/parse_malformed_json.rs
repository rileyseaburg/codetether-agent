//! Regression for parse_malformed_json.

use super::*;

#[test]
fn parse_malformed_json() {
    let json = "not valid json at all";
    let payload = FinalPayload::parse(json);
    match payload {
        FinalPayload::Malformed { raw, error } => {
            assert_eq!(raw, "not valid json at all");
            assert!(error.contains("JSON parse error"));
        }
        _ => panic!("Expected Malformed payload"),
    }
}
