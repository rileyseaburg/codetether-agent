use codetether_agent::provider::{CompletionRequest, bedrock};

#[test]
fn resolves_fable_alias() {
    assert_eq!(
        bedrock::resolve_model_id("fable"),
        "global.anthropic.claude-fable-5"
    );
    assert_eq!(
        bedrock::resolve_model_id("claude-fable-5"),
        "global.anthropic.claude-fable-5"
    );
    assert_eq!(
        bedrock::resolve_model_id("us.anthropic.claude-fable-5"),
        "global.anthropic.claude-fable-5"
    );
}

#[test]
fn fable_request_omits_temperature() {
    let req = CompletionRequest {
        model: "fable".into(),
        messages: vec![],
        tools: vec![],
        temperature: Some(0.7),
        top_p: None,
        max_tokens: None,
        stop: vec![],
    };
    let body = bedrock::build_converse_body(&req, "global.anthropic.claude-fable-5");

    assert!(body["inferenceConfig"].get("temperature").is_none());
}

#[test]
fn opus_48_request_omits_temperature() {
    let req = CompletionRequest {
        model: "claude-opus-4-8".into(),
        messages: vec![],
        tools: vec![],
        temperature: Some(0.7),
        top_p: None,
        max_tokens: None,
        stop: vec![],
    };
    let body = bedrock::build_converse_body(&req, "us.anthropic.claude-opus-4-8");

    assert!(body["inferenceConfig"].get("temperature").is_none());
}
