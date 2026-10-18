use super::{ContentPart, fixtures::*, serializers};
#[test]
fn legacy_chat_leading_packed_image_belongs_only_to_first_result() {
    for serialize in serializers() {
        let mut packed = result("call-a");
        let leading_url = "https://example.test/leading.png";
        packed.content.insert(
            0,
            ContentPart::Image {
                url: leading_url.into(),
                mime_type: None,
            },
        );
        packed.content.extend(result("call-b").content);
        let output = serialize(&[packed]);
        assert_eq!(output.len(), 5);
        assert_eq!(output[0]["tool_call_id"], "call-a");
        assert_eq!(output[1]["tool_call_id"], "call-b");
        for (index, id) in ["call-a", "call-a", "call-b"].into_iter().enumerate() {
            assert_eq!(output[index + 2]["role"], "user");
            assert_eq!(
                output[index + 2]["content"][0]["text"],
                format!("Image from tool result: {id}")
            );
        }
        assert_eq!(output[2]["content"][1]["image_url"]["url"], leading_url);
        assert_eq!(output[3]["content"][1]["image_url"]["url"], PIXELS);
        assert_eq!(output[4]["content"][1]["image_url"]["url"], PIXELS);
    }
}
