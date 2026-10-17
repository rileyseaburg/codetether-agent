//! Migrated application stream regression.
use super::*;
#[test]
fn stream_response_without_usage_or_choices_is_ok() {
    // A bare [DONE]-adjacent frame with neither choices nor usage must
    // still deserialize so the empty-stream guard can run.
    let parsed: ZaiStreamResponse =
        serde_json::from_str(r#"{}"#).expect("empty frame should parse");
    assert!(parsed.choices.is_empty());
    assert!(parsed.usage.is_none());
}
