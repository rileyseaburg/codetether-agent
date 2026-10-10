mod common;
use codetether_companion_protocol::*;
use common::{fixtures, round_trip};

#[test]
fn relay_receipts_and_requests_keep_exact_field_names() {
    let f = fixtures();
    round_trip::<SessionInput>(&f["session_input"]);
    round_trip::<SessionReceipt>(&f["session_receipt"]);
    round_trip::<PairRequest>(&f["pair_request"]);
    round_trip::<PairReceipt>(&f["pair_receipt"]);
    round_trip::<CaptureRequest>(&f["capture_request"]);
    round_trip::<CaptureRequestReceipt>(&f["request_receipt"]);
}

#[test]
fn device_commands_preserve_required_null_without_owner_question() {
    let f = fixtures();
    round_trip::<DeviceCommand>(&f["command_idle"]);
    round_trip::<DeviceCommand>(&f["command_pending"]);
    assert!(serde_json::from_str::<DeviceCommand>("{}").is_err());
    assert!(serde_json::from_str::<DeviceCommand>(r#"{"request_id":3}"#).is_err());
}

#[test]
fn response_acknowledgements_and_errors_match_relay() {
    let f = fixtures();
    round_trip::<Accepted>(&f["accepted"]);
    round_trip::<Paused>(&f["paused"]);
    round_trip::<Stopped>(&f["stopped"]);
    round_trip::<ErrorResponse>(&f["error"]);
}

#[test]
fn serde_types_do_not_claim_semantic_validation() {
    let mut input = fixtures()["session_input"].clone();
    input["interval_seconds"] = 1.into();
    assert!(serde_json::from_value::<SessionInput>(input.clone()).is_ok());
    input["interval_seconds"] = 1.5.into();
    assert!(serde_json::from_value::<SessionInput>(input).is_err());
}
