//! Stable PTY request wire-shape regression.

#[test]
fn tail_request_has_a_stable_wire_shape() {
    let value = serde_json::json!({"action": "tail", "window_id": 7});
    let request: super::ProgramRequest = serde_json::from_value(value).unwrap();
    assert!(matches!(
        request,
        super::ProgramRequest::Tail { window_id: 7 }
    ));
}
