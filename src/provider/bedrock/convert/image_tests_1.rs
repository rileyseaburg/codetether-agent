//! Serialization regression tests for native Converse image blocks.
use super::super::image_test_support::*;

#[test]
fn tool_images_merge_without_losing_call_ids_or_error_text() {
    let out = convert(&transcript());
    assert_eq!(out.as_array().unwrap().len(), 2);
    let parts = &out[1]["content"];
    assert_eq!(parts.as_array().unwrap().len(), 2);
    for (i, id, data) in [(0, "a", "YQ=="), (1, "b", "Yg==")] {
        assert_eq!(parts[i]["toolResult"]["toolUseId"], id);
        assert_eq!(
            parts[i]["toolResult"]["content"][1]["image"]["source"]["bytes"],
            data
        );
        assert_eq!(
            parts[i]["toolResult"]["content"][1]["image"]["format"],
            "png"
        );
    }
    assert_eq!(
        parts[1]["toolResult"]["content"][0]["text"],
        "Error: second capture partial"
    );
}
