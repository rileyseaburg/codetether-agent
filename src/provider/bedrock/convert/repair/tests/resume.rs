use crate::provider::bedrock::convert::convert_messages;
use crate::provider::{ContentPart, Message, Role};

#[test]
fn resumed_session_places_repair_before_prompt() {
    let input = vec![
        Message {
            role: Role::Assistant,
            content: vec![tool_call("tooluse_ifUHK")],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "Session".into(),
            }],
        },
    ];
    let (_, messages) = convert_messages(&input);
    let result = &messages[1]["content"][0]["toolResult"];
    assert_eq!(result["toolUseId"], "tooluse_ifUHK");
    assert_eq!(result["status"], "error");
    assert_eq!(messages[1]["content"][1]["text"], "Session");
}

fn tool_call(id: &str) -> ContentPart {
    ContentPart::ToolCall {
        id: id.into(),
        name: "bash".into(),
        arguments: "{}".into(),
        thought_signature: None,
    }
}
