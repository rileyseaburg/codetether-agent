//! Tool attachments follow all parallel replies, never enter tool content.

use super::support::serialized;
#[path = "tool_image_fixture.rs"]
mod fixture;

#[test]
fn tool_images_are_flushed_at_end_of_history() {
    let mut history = fixture::history(false);
    history.pop();
    for messages in serialized(&history) {
        assert_eq!(messages.len(), 5);
        assert_eq!(messages[2]["tool_call_id"], "call_b");
        assert_eq!(messages[4]["content"][1]["type"], "image_url");
    }
}

#[test]
fn tool_images_follow_all_replies_with_origin_labels() {
    for packed in [false, true] {
        let history = fixture::history(packed);
        let original = serde_json::to_value(&history).unwrap();
        for messages in serialized(&history) {
            assert_eq!(messages.len(), 6);
            for (offset, id) in ["call_a", "call_b"].into_iter().enumerate() {
                assert_eq!(messages[1 + offset]["role"], "tool");
                assert_eq!(messages[1 + offset]["tool_call_id"], id);
                assert_eq!(messages[1 + offset]["content"], format!("output {id}"));
                let attachment = &messages[3 + offset];
                assert_eq!(attachment["role"], "user");
                assert_eq!(
                    attachment["content"][0]["text"],
                    format!("Image attached to tool output for tool_call_id: {id}")
                );
                assert_eq!(
                    attachment["content"][1]["image_url"]["url"],
                    "data:image/png;base64,iVBORw0KGgo="
                );
            }
            assert_eq!(messages[5]["content"], "continue");
        }
        assert_eq!(serde_json::to_value(&history).unwrap(), original);
    }
}
