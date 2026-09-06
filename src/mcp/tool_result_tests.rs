//! MCP image blocks use mimeType and preserve local tool success/error status.

use super::convert;
use crate::mcp::{CallToolResult, ToolContent};
use crate::tool::{ToolResult, result_images};
use serde_json::json;

#[test]
fn mcp_result_images_roundtrip_standard_wire_fields() {
    for success in [true, false] {
        let mut result = ToolResult::success("pixels captured").with_metadata(
            "image_data_url",
            json!([
                result_images::encoded(&[1, 2, 3], "image/png"),
                result_images::encoded(&[4, 5, 6], "image/jpeg"),
            ]),
        );
        result.success = success;
        let wire = serde_json::to_value(convert(result)).unwrap();
        assert_eq!(wire["isError"], !success);
        assert_eq!(wire["content"][0]["text"], "pixels captured");
        assert_eq!(
            wire["content"][1],
            json!({"type":"image", "data":"AQID", "mimeType":"image/png"})
        );
        let decoded: CallToolResult = serde_json::from_value(wire).unwrap();
        assert_eq!(decoded.content.len(), 3);
        assert!(
            matches!(&decoded.content[2], ToolContent::Image { data, mime_type }
            if data == "BAUG" && mime_type == "image/jpeg")
        );
    }
}

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
