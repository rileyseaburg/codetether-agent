//! Mocked image-only MCP result regression.

use super::result;
use crate::mcp::{CallToolResult, ToolContent};

#[test]
fn mcp_image_only_has_no_text_placeholder() {
    for is_error in [false, true] {
        let converted = result(CallToolResult {
            content: vec![ToolContent::Image {
                data: "AQID".into(),
                mime_type: "image/png".into(),
            }],
            is_error,
        });
        assert_eq!(converted.success, !is_error);
        assert!(converted.output.is_empty());
        assert_eq!(
            converted.metadata["image_data_url"][0]["data_url"],
            "data:image/png;base64,AQID"
        );
    }
}
