//! Prompt-policy tests for the session-recall tool description.

#[test]
fn description_respects_user_source_and_access_rules() {
    let description = super::tool_struct::DESCRIPTION;
    assert!(description.contains("source and access instructions first"));
    assert!(description.contains("persists until explicitly revoked"));
    assert!(description.contains("user-designated repository sources"));
    assert!(!description.contains("Call this whenever"));
}

#[test]
fn evidence_is_the_default_recall_mode() {
    let args = super::args::RecallArgs::parse(&serde_json::json!({"query": "decision"}))
        .expect("valid recall args");
    assert!(matches!(args.mode, super::args::RecallMode::Evidence));
}
