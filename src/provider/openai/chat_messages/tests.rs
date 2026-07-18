use super::convert;
use crate::provider::{ContentPart, Message, Role};
use async_openai::types::chat::ChatCompletionRequestMessage;

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
