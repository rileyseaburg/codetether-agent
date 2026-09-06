use super::{ContentPart, Role, fixtures::*, json, serializers};
#[test]
fn legacy_chat_text_only_content_stays_a_string() {
    for serialize in serializers() {
        let input = [message(Role::User, vec![text("one"), text("two")])];
        assert_eq!(
            serialize(&input),
            vec![json!({"role":"user", "content":"one\ntwo"})]
        );
    }
}
#[test]
fn legacy_chat_google_thought_signature_survives_decoration() {
    use crate::provider::google::GoogleProvider;
    let input = [
        message(
            Role::Assistant,
            vec![ContentPart::ToolCall {
                id: "call-a".into(),
                name: "screenshot".into(),
                arguments: "{}".into(),
                thought_signature: Some("signature".into()),
            }],
        ),
        result("call-a"),
    ];
    let output = GoogleProvider::convert_messages(&input);
    assert_eq!(
        output[0]["tool_calls"][0]["extra_content"]["google"]["thought_signature"],
        "signature"
    );
    assert_eq!(output[1]["tool_call_id"], "call-a");
    assert_eq!(output[2]["content"][1]["image_url"]["url"], PIXELS);
}
