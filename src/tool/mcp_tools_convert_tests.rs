//! Mocked MCP response tests for image preservation on success and error.

use super::result;
use crate::mcp::CallToolResult;
use serde_json::json;

#[test]
fn mcp_images_preserve_base64_order_and_error_status() {
    for is_error in [false, true] {
        let response: CallToolResult = serde_json::from_value(json!({
            "isError": is_error,
            "content": [
                {"type": "text", "text": "before"},
                {"type": "image", "data": "AQID", "mime_type": "image/png"},
                {"type": "resource", "resource": {"uri": "file:///x", "text": "body"}},
                {"type": "image", "data": "BAU=", "mime_type": "image/jpeg"},
                {"type": "text", "text": "after"}
            ]
        }))
        .unwrap();
        let converted = result(response);
        assert_eq!(converted.success, !is_error);
        let lines: Vec<&str> = converted.output.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "before");
        assert_eq!(lines[2], "after");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(lines[1]).unwrap(),
            json!({"uri": "file:///x", "text": "body"})
        );
        assert_eq!(
            converted.metadata["image_data_url"],
            json!([
                {"data_url": "data:image/png;base64,AQID", "mime_type": "image/png"},
                {"data_url": "data:image/jpeg;base64,BAU=", "mime_type": "image/jpeg"}
            ])
        );
    }
}
