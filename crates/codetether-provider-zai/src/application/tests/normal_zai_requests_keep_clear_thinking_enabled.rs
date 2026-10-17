//! Migrated application provider regression.
use super::*;
#[test]
fn normal_zai_requests_keep_clear_thinking_enabled() {
    let request = CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "Explain the tradeoffs.".to_string(),
            }],
        }],
        tools: vec![],
        model: "glm-5.1".to_string(),
        temperature: None,
        top_p: None,
        max_tokens: Some(1024),
        stop: vec![],
    };

    assert!(!should_disable_thinking_for_request(&request));
    assert_eq!(thinking_config_for_request(&request)["type"], "enabled");
    assert_eq!(
        thinking_config_for_request(&request)["clear_thinking"],
        true
    );
}
