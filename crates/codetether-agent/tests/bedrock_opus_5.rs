use codetether_agent::provider::{CompletionRequest, ContentPart, Message, Role, bedrock};

#[test]
fn opus_5_request_omits_deprecated_temperature() {
    let request = CompletionRequest {
        model: "us.anthropic.claude-opus-5".into(),
        messages: vec![],
        tools: vec![],
        temperature: Some(0.7),
        top_p: None,
        max_tokens: Some(8_192),
        stop: vec![],
    };

    let body = bedrock::build_converse_body(&request, "us.anthropic.claude-opus-5");

    assert!(body["inferenceConfig"].get("temperature").is_none());
}

#[test]
fn opus_5_request_ends_with_user_turn() {
    let mut request = CompletionRequest {
        model: "us.anthropic.claude-opus-5".into(),
        messages: vec![],
        tools: vec![],
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: vec![],
    };
    request.messages.push(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: "Working".into(),
        }],
    });

    let body = bedrock::build_converse_body(&request, "us.anthropic.claude-opus-5");

    assert_eq!(body["messages"][1]["role"], "user");
    assert_eq!(body["messages"][1]["content"][0]["text"], "Continue.");
}
