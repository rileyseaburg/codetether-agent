use super::{ContentPart, Role, fixtures::*, serializers};
#[test]
fn legacy_chat_tool_images_follow_all_consecutive_replies() {
    for serialize in serializers() {
        let mut source = vec![
            result("call-a"),
            result("call-b"),
            message(Role::Assistant, vec![text("done")]),
        ];
        let second_url = "https://example.test/second.png";
        source[1].content[1] = ContentPart::Image {
            url: second_url.into(),
            mime_type: None,
        };
        let output = serialize(&source);
        assert_eq!(output.len(), 5);
        for (index, id) in ["call-a", "call-b"].into_iter().enumerate() {
            assert_eq!(output[index]["role"], "tool");
            assert_eq!(output[index]["tool_call_id"], id);
            assert_eq!(output[index]["content"], format!("result {id}"));
            assert_eq!(output[index + 2]["role"], "user");
            assert!(
                output[index + 2]["content"][0]["text"]
                    .as_str()
                    .unwrap()
                    .contains(id)
            );
            assert_eq!(
                output[index + 2]["content"][1]["image_url"]["url"],
                [PIXELS, second_url][index]
            );
        }
        assert_eq!(output[4]["role"], "assistant");
        assert!(source[..2].iter().all(|message| message.role == Role::Tool));
        assert_eq!(serialize(&source[..2]).len(), 4);
    }
}
