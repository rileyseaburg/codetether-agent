use crate::PointerHandle;

#[test]
fn pointer_handle_round_trips_through_serde() {
    let handle = PointerHandle {
        bucket: "b".into(),
        key: "k".into(),
        byte_range: Some((16, 64)),
    };
    let json = serde_json::to_string(&handle).unwrap();
    let back: PointerHandle = serde_json::from_str(&json).unwrap();
    assert_eq!(back, handle);
}
