//! Mocked tool lifecycle coverage for call-associated image recording.

use super::{image_test_state::state, state::ToolCall, tools};
use codetether_provider_core::{ContentPart, Role};
use codetether_tool_core::ToolResult;
use serde_json::json;

#[tokio::test]
async fn swarm_tool_images_survive_success_failure_and_text_truncation() {
    let first = json!({"data_url": "data:image/png;base64,YQ==", "mime_type": "image/png"});
    let second = json!({"data_url": "data:image/png;base64,Yg==", "mime_type": "image/png"});
    for success in [true, false] {
        for (images, count) in [(first.clone(), 1), (json!([first, second]), 2)] {
            let mut result =
                ToolResult::success("x".repeat(12_000)).with_metadata("image_data_url", images);
            result.success = success;
            let expected = codetether_tool_core::images::content(Some(&result.metadata));
            let mut state = state(result);
            let call = ToolCall {
                id: "original-call".into(),
                name: "image".into(),
                arguments: "{}".into(),
            };
            assert!(!tools::execute(&mut state, vec![call]).await.unwrap());
            assert_eq!(state.inner.conversation.messages.len(), 3);
            let message = &state.inner.conversation.messages[2];
            assert!(matches!(message.role, Role::Tool));
            assert_eq!(expected.len(), count);
            assert_eq!(message.content.len(), 1 + expected.len());
            let ContentPart::ToolResult {
                tool_call_id,
                content,
            } = &message.content[0]
            else {
                panic!("first part must be the original tool result");
            };
            assert_eq!(tool_call_id, "original-call");
            assert!(content.len() < 12_000);
            assert!(!content.contains("data:image"));
            assert_eq!(content.contains("Tool error:"), !success);
            assert_eq!(
                serde_json::to_value(&message.content[1..]).unwrap(),
                serde_json::to_value(expected).unwrap()
            );
        }
    }
}
