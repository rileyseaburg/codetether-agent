//! Existing shared-context regression.
use super::*;

#[test]
fn parse_json_payload_extracts_embedded_object() {
    let parsed = parse_json_payload::<ContextDelta>(
        "noise {\"facts\":[\"a\"],\"decisions\":[],\"risks\":[],\"next_actions\":[],\"evidence\":[]} trailing",
    );
    assert!(parsed.is_some());
}
