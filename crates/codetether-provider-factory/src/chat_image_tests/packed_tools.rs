use super::{ContentPart, Role, fixtures::*, serializers};
#[test]
fn legacy_chat_packed_tool_replies_keep_each_image_with_its_call() {
    for serialize in serializers() {
        let mut packed = result("call-a");
        let mut second = result("call-b");
        let second_url = "https://example.test/second.png";
        second.content[1] = ContentPart::Image {
            url: second_url.into(),
            mime_type: None,
        };
        packed.content.extend(second.content);
        let output = serialize(&[packed]);
        assert_eq!(output.len(), 4);
        for (index, id) in ["call-a", "call-b"].into_iter().enumerate() {
            assert_eq!(output[index]["role"], "tool");
            assert_eq!(output[index]["tool_call_id"], id);
            assert_eq!(output[index]["content"], format!("result {id}"));
            assert_eq!(
                output[index + 2]["content"][0]["text"],
                format!("Image from tool result: {id}")
            );
            assert_eq!(
                output[index + 2]["content"][1]["image_url"]["url"],
                [PIXELS, second_url][index]
            );
        }
        assert_eq!(
            serialize(&[message(Role::User, vec![text("next")])]).len(),
            1
        );
    }
}
