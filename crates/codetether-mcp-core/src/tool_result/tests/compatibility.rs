//! Accept legacy image fields and reject unsupported image URL transport.
use super::*;
#[test]
fn mcp_image_accepts_legacy_field_and_rejects_unrepresentable_urls() {
    let legacy: ToolContent = serde_json::from_value(json!({
        "type":"image", "data":"AQID", "mime_type":"image/png"
    }))
    .unwrap();
    assert_eq!(
        serde_json::to_value(legacy).unwrap()["mimeType"],
        "image/png"
    );
    let result = convert(ToolResult::success("reference").with_metadata(
        "image_data_url",
        json!({"data_url":"https://example.com/image.png"}),
    ));
    assert!(result.is_error);
}
