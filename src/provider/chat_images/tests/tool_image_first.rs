use super::{fixtures::*, serializers};
#[test]
fn legacy_chat_tool_image_before_result_retains_call_id() {
    for serialize in serializers() {
        let mut source = result("call-first");
        source.content.rotate_right(1);
        let output = serialize(&[source]);
        assert_eq!(output[0]["role"], "tool");
        assert_eq!(output[0]["tool_call_id"], "call-first");
        assert_eq!(output[0]["content"], "result call-first");
        assert_eq!(output[1]["role"], "user");
        assert_eq!(output[1]["content"][1]["image_url"]["url"], PIXELS);
    }
}
