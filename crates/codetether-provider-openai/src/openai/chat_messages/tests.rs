#[path = "test_support.rs"]
mod support;
#[path = "tool_image_tests.rs"]
mod tool_images;
#[path = "user_image_tests.rs"]
mod user_images;
use super::convert;
use async_openai::types::chat::ChatCompletionRequestMessage;
use codetether_provider_core::{ContentPart, Message, Role};

#[test]
fn developer_history_uses_native_openai_role() {
    let messages = convert(&[Message {
        role: Role::Developer,
        content: vec![ContentPart::Text {
            text: "interrupted".into(),
        }],
    }])
    .unwrap();
    assert!(matches!(
        messages.as_slice(),
        [ChatCompletionRequestMessage::Developer(_)]
    ));
}
