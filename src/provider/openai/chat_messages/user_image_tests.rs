//! Ordered user images and text-only compatibility on both transports.

use super::support::{image, serialized, text};
use crate::provider::{Message, Role};
use serde_json::json;

#[test]
fn mixed_and_image_only_user_content_keeps_native_image_parts() {
    let image_json = json!({"type": "image_url", "image_url": {
        "url": "data:image/png;base64,iVBORw0KGgo="
    }});
    for (content, expected) in [
        (vec![image()], json!([image_json.clone()])),
        (
            vec![text("before"), image(), text("after"), image()],
            json!([
                {"type": "text", "text": "before"}, image_json.clone(),
                {"type": "text", "text": "after"}, image_json
            ]),
        ),
    ] {
        for messages in serialized(&[Message {
            role: Role::User,
            content,
        }]) {
            assert_eq!(messages.len(), 1);
            assert_eq!(messages[0]["role"], "user");
            assert_eq!(messages[0]["content"], expected);
        }
    }
}

#[test]
fn text_only_content_stays_a_joined_string() {
    for role in [Role::User, Role::System, Role::Developer] {
        for messages in serialized(&[Message {
            role,
            content: vec![text("one"), text("two")],
        }]) {
            assert_eq!(messages[0]["content"], "one\ntwo");
        }
    }
}
