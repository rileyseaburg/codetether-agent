//! Migrated application provider regression.
use super::*;
#[test]
fn long_low_budget_zai_requests_keep_thinking_enabled() {
    let request = CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: format!(
                    "Summarize the following material. {}",
                    "context paragraph. ".repeat(40)
                ),
            }],
        }],
        tools: vec![],
        model: "glm-5.1".to_string(),
        temperature: None,
        top_p: None,
        max_tokens: Some(128),
        stop: vec![],
    };

    assert!(!should_disable_thinking_for_request(&request));
    assert_eq!(thinking_config_for_request(&request)["type"], "enabled");
}
