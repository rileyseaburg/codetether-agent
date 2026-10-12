use codetether_provider_core::{FinishReason, Role, Usage};
use serde_json::json;

#[test]
fn role_wire_names_remain_unchanged() {
    for (role, name) in [
        (Role::System, "system"),
        (Role::Developer, "developer"),
        (Role::User, "user"),
        (Role::Assistant, "assistant"),
        (Role::Tool, "tool"),
    ] {
        assert_eq!(serde_json::to_value(role).unwrap(), json!(name));
        assert_eq!(serde_json::from_value::<Role>(json!(name)).unwrap(), role);
    }
}

#[test]
fn finish_reason_wire_names_remain_unchanged() {
    for (reason, name) in [
        (FinishReason::Stop, "stop"),
        (FinishReason::Length, "length"),
        (FinishReason::ToolCalls, "tool_calls"),
        (FinishReason::ContentFilter, "content_filter"),
        (FinishReason::Error, "error"),
    ] {
        assert_eq!(serde_json::to_value(reason).unwrap(), json!(name));
        assert_eq!(
            serde_json::from_value::<FinishReason>(json!(name)).unwrap(),
            reason
        );
    }
}

#[test]
fn usage_preserves_explicit_totals_and_optional_cache_counts() {
    let wire = json!({"prompt_tokens": 10, "completion_tokens": 5, "total_tokens": 0});
    let usage: Usage = serde_json::from_value(wire).unwrap();
    assert_eq!(usage.total_tokens, 0);
    assert_eq!(usage.cache_read_tokens, None);
    assert_eq!(usage.cache_write_tokens, None);
    assert_eq!(
        serde_json::to_value(usage).unwrap(),
        json!({
            "prompt_tokens": 10, "completion_tokens": 5, "total_tokens": 0,
            "cache_read_tokens": null, "cache_write_tokens": null,
        })
    );
}
