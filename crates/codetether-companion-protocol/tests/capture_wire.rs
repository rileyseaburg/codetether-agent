mod common;
use codetether_companion_protocol::{Capture, CaptureTrigger};
use common::{fixtures, round_trip};

#[test]
fn legacy_and_all_trigger_uploads_match_relay() {
    let f = fixtures();
    round_trip::<Capture>(&f["frame_legacy"]);
    for frame in f["frames"].as_array().unwrap() {
        round_trip::<Capture>(frame);
    }
    let manual: Capture = serde_json::from_value(f["frames"][3].clone()).unwrap();
    assert_eq!(manual.trigger, Some(CaptureTrigger::Manual));
    assert_eq!(
        manual.request_id.unwrap(),
        f["request_receipt"]["request_id"]
    );
}

#[test]
fn invalid_frame_shapes_fail_closed() {
    for frame in fixtures()["invalid_frames"].as_array().unwrap() {
        assert!(serde_json::from_value::<Capture>(frame.clone()).is_err());
    }
}
