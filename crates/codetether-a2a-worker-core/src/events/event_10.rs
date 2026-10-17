//! Structured event regression case.
use super::*;

#[test]
fn multibyte_output_truncation_stays_valid() {
    let text = "é".repeat(MAX_OUTPUT_BYTES);
    let event = tool_result_event("read", true, &text, None);

    // Serializing proves the carried string is still valid UTF-8.
    assert!(serde_json::to_string(&event).is_ok());
    assert_eq!(
        event["payload"]["output_truncated"],
        serde_json::json!(true)
    );
}
