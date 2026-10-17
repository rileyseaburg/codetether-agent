//! Serialization regression tests for native Anthropic image blocks.
use super::super::image_test_support::*;

#[test]
fn tool_images_keep_distinct_call_ids_and_error_text() {
    let out = convert(&transcript());
    assert_eq!(out.as_array().unwrap().len(), 2);
    let parts = &out[1]["content"];
    assert_eq!(parts.as_array().unwrap().len(), 2);
    for (i, id, data) in [(0, "a", "YQ=="), (1, "b", "Yg==")] {
        assert_eq!(parts[i]["type"], "tool_result");
        assert_eq!(parts[i]["tool_use_id"], id);
        assert_eq!(parts[i]["content"][1]["source"]["data"], data);
        assert_eq!(parts[i]["content"][1]["source"]["media_type"], "image/png");
    }
    assert_eq!(
        parts[1]["content"][0]["text"],
        "Error: second capture partial"
    );
}
