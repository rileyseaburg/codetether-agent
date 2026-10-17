//! Regression for malformed_payload_is_serializable.

use super::*;

#[test]
fn malformed_payload_is_serializable() {
    let payload = FinalPayload::Malformed {
        raw: "oops".to_string(),
        error: "parse error".to_string(),
    };
    let json = serde_json::to_string(&payload).expect("malformed payload should serialize");
    assert!(json.contains("\"kind\":\"malformed\""));
    assert!(json.contains("\"raw\":\"oops\""));
    assert!(json.contains("\"error\":\"parse error\""));
}
