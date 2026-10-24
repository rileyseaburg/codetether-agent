use super::inner;
use crate::support::state;
use codetether_gemini_web_protocol::images::build_payload;
use serde_json::json;

#[test]
fn image_payload_continues_and_rejects_invalid_input() {
    let value = inner("make it blue", true);
    assert_eq!(
        value[2],
        json!([
            "c_test", "r_test", "rc_test", null, null, null, null, null, null, ""
        ])
    );
    assert_eq!(
        value[71],
        json!(["c_test", null, null, null, null, null, null, ""])
    );
    assert!(build_payload("\t \n", None).is_err());
    let mut invalid = state();
    invalid.choice_id = "secret URL".into();
    assert_eq!(
        build_payload("draw", Some(&invalid))
            .unwrap_err()
            .to_string(),
        "image_state_invalid"
    );
}
