use crate::provider::{ContentPart, Role};
use crate::tool::ToolResult;
use serde_json::json;

#[tokio::test]
async fn generated_image_stays_with_tool_result() {
    let mut session = crate::session::Session::new().await.unwrap();
    let call = ContentPart::ToolCall {
        id: "call-1".into(),
        name: "image_gen".into(),
        arguments: "{}".into(),
        thought_signature: None,
    };
    let result = ToolResult::success("generated").with_metadata(
        "image_data_url",
        json!({
            "data_url": "data:image/png;base64,eA==",
            "mime_type": "image/png"
        }),
    );
    super::tool_result_record::record_results(&mut session, vec![call], vec![result]);
    let message = session.messages.last().unwrap();
    assert!(matches!(message.role, Role::Tool));
    assert!(matches!(
        message.content.as_slice(),
        [ContentPart::ToolResult { .. }, ContentPart::Image { url, .. }]
            if url == "data:image/png;base64,eA=="
    ));
}
