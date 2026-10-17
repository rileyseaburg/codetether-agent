//! Fault contract regression.
use crate::fault::Fault;

#[test]
fn fault_round_trips_through_serde() {
    let f = Fault::Denied {
        reason: "policy".into(),
    };
    let json = serde_json::to_string(&f).unwrap();
    assert!(json.contains("\"kind\":\"denied\""));
    let back: Fault = serde_json::from_str(&json).unwrap();
    assert_eq!(back, f);
}
