//! Tool results remain paired when a Fable streaming answer is replayed.
use super::*;

#[test]
fn fable_stream_followup_keeps_tool_history_and_synthesized_schema() {
    let provider = BedrockProvider::new("fixture-token".into()).unwrap();
    let mut request = request();
    request.messages.extend([
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "lookup-1".into(),
                name: "lookup".into(),
                arguments: "{\"n\":1}".into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "lookup-1".into(),
                content: "found".into(),
            }],
        },
    ]);
    let body = provider.build_converse_body(&request, &request.model);
    assert_eq!(body["messages"][1]["role"], "assistant");
    assert_eq!(
        body["messages"][1]["content"][0]["toolUse"]["toolUseId"],
        "lookup-1"
    );
    assert_eq!(body["messages"][2]["role"], "user");
    assert_eq!(
        body["messages"][2]["content"][0]["toolResult"]["toolUseId"],
        "lookup-1"
    );
    assert_eq!(body["toolConfig"]["tools"][0]["toolSpec"]["name"], "lookup");
    let messages = body["messages"].as_array().unwrap();
    assert!(codetether_bedrock_protocol::audit::unpaired(messages).is_none());
}
