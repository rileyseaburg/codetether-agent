//! Structured event regression case.
use super::*;

#[test]
fn long_output_is_truncated_explicitly() {
    let long = "x".repeat(MAX_OUTPUT_BYTES * 2);
    let event = tool_result_event("bash", true, &long, None);

    assert_eq!(
        event["payload"]["output_truncated"],
        serde_json::json!(true)
    );
    assert_eq!(
        event["payload"]["output_bytes"],
        serde_json::json!(long.len())
    );
    let carried = event["payload"]["output"].as_str().unwrap();
    assert!(carried.len() <= MAX_OUTPUT_BYTES, "{}", carried.len());
}
