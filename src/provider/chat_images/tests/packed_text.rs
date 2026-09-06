use super::{ContentPart, fixtures::*, serializers};
#[test]
fn legacy_chat_text_only_packed_replies_stay_before_pending_images() {
    for serialize in serializers() {
        let mut packed = result("call-b");
        packed.content.extend(result("call-c").content);
        packed
            .content
            .retain(|part| !matches!(part, ContentPart::Image { .. }));
        let output = serialize(&[result("call-a"), packed]);
        assert_eq!(output.len(), 4);
        for (index, id) in ["call-a", "call-b", "call-c"].into_iter().enumerate() {
            assert_eq!(output[index]["role"], "tool");
            assert_eq!(output[index]["tool_call_id"], id);
            assert_eq!(output[index]["content"], format!("result {id}"));
        }
        assert_eq!(output[3]["role"], "user");
        assert_eq!(
            output[3]["content"][0]["text"],
            "Image from tool result: call-a"
        );
        assert_eq!(output[3]["content"][1]["image_url"]["url"], PIXELS);
    }
}
