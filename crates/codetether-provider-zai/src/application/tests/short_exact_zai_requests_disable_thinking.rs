//! Migrated application provider regression.
use super::*;
#[test]
fn short_exact_zai_requests_disable_thinking() {
    let request = CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "Reply with exactly: api-model-call-ok".to_string(),
            }],
        }],
        tools: vec![],
        model: "glm-5.1".to_string(),
        temperature: None,
        top_p: None,
        max_tokens: Some(20),
        stop: vec![],
    };

    assert!(should_disable_thinking_for_request(&request));
    assert_eq!(
        thinking_config_for_request(&request),
        json!({"type": "disabled"})
    );
}
