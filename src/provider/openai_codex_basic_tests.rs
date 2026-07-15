#[test]
fn test_generate_pkce() {
    let pkce = OpenAiCodexProvider::generate_pkce();
    assert!(!pkce.verifier.is_empty());
    assert!(!pkce.challenge.is_empty());
    assert_ne!(pkce.verifier, pkce.challenge);
}

#[test]
fn test_generate_state() {
    let state = OpenAiCodexProvider::generate_state();
    assert_eq!(state.len(), 32);
}

#[test]
fn formats_scope_error_with_actionable_message() {
    let body = r#"{"error":{"message":"Missing scopes: model.request"}}"#;
    let msg = OpenAiCodexProvider::format_openai_api_error(
        StatusCode::UNAUTHORIZED,
        body,
        "gpt-5.3-codex",
    );
    assert!(msg.contains("model.request"));
    assert!(msg.contains("codetether auth codex"));
}

#[test]
fn maps_fast_model_alias_to_priority_service_tier() {
    let (model, level, tier) =
        OpenAiCodexProvider::resolve_model_and_reasoning_effort_and_service_tier(
            "gpt-5.4-fast:high",
        );
    assert_eq!(model, "gpt-5.4");
    assert_eq!(level.map(ThinkingLevel::as_str), Some("high"));
    assert_eq!(tier.map(CodexServiceTier::as_str), Some("priority"));

    let (model, level, tier) =
        OpenAiCodexProvider::resolve_model_and_reasoning_effort_and_service_tier(
            "gpt-5.5-fast:high",
        );
    assert_eq!(model, "gpt-5.5");
    assert_eq!(level.map(ThinkingLevel::as_str), Some("high"));
    assert_eq!(tier.map(CodexServiceTier::as_str), Some("priority"));
}
