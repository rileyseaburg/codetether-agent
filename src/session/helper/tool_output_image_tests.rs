//! Structured images survive success/failure feedback and text compaction.

use super::tool_output::tool_result_with_metadata;
use crate::provider::{ContentPart, Role};
use crate::tool::{ToolResult, result_images};
use serde_json::json;

#[test]
fn session_tool_images_survive_compaction_and_failures() {
    let first = result_images::encoded(&[1, 2, 3], "image/png");
    let second = result_images::encoded(&[4, 5, 6], "image/jpeg");
    for success in [true, false] {
        let result = ToolResult::success("x".repeat(12_000))
            .with_metadata("image_data_url", json!([first, second]));
        let message = tool_result_with_metadata(
            "image-call".into(),
            "image",
            success,
            result.output,
            Some(&result.metadata),
        );
        assert_eq!(message.role, Role::Tool);
        assert_eq!(message.content.len(), 3);
        let ContentPart::ToolResult {
            tool_call_id,
            content,
        } = &message.content[0]
        else {
            panic!("missing tool result")
        };
        assert_eq!(tool_call_id, "image-call");
        assert!(content.len() < 12_000);
        assert!(!content.contains("base64"));
        assert!(matches!(&message.content[1], ContentPart::Image { url, .. }
            if url == "data:image/png;base64,AQID"));
        assert!(matches!(&message.content[2], ContentPart::Image { url, .. }
            if url == "data:image/jpeg;base64,BAUG"));
    }
}
