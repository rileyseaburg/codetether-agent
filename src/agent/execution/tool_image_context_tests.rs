use crate::provider::{ContentPart, Message, Role};
use serde_json::json;

#[tokio::test]
async fn image_generation_receives_recent_conversation_images() {
    let mut session = crate::session::Session::new().await.unwrap();
    for url in ["first", "second"] {
        session.messages.push(Message {
            role: Role::User,
            content: vec![ContentPart::Image {
                url: url.into(),
                mime_type: Some("image/png".into()),
            }],
        });
    }
    let value = super::tool_image_context::enrich("image_gen", "call-1", &session, json!({}));
    assert_eq!(value["__ct_recent_images"], json!(["first", "second"]));
    assert_eq!(value["__ct_tool_call_id"], "call-1");
}

#[tokio::test]
async fn unrelated_tools_do_not_receive_image_payloads() {
    let session = crate::session::Session::new().await.unwrap();
    let value = super::tool_image_context::enrich("bash", "call-1", &session, json!({}));
    assert!(value.get("__ct_recent_images").is_none());
    assert!(value.get("__ct_tool_call_id").is_none());
}
