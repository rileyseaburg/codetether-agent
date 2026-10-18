//! Original session provider-policy regression case.
use crate::should_retry_missing_native_tool_call;
use codetether_provider_core::ToolDefinition;
use serde_json::json;

#[test]
fn should_retry_missing_native_tool_call_targets_minimax_only() {
    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "shell".to_string(),
        parameters: json!({"type":"object"}),
    }];

    assert!(should_retry_missing_native_tool_call(
        "minimax",
        "MiniMax-M2.5",
        0,
        &tools,
        "I'll use bash to inspect the repository first.",
        false,
        1,
    ));
    assert!(!should_retry_missing_native_tool_call(
        "anthropic",
        "claude-sonnet-4",
        0,
        &tools,
        "I'll use bash to inspect the repository first.",
        false,
        1,
    ));
    assert!(!should_retry_missing_native_tool_call(
        "minimax",
        "MiniMax-M2.5",
        1,
        &tools,
        "I'll use bash to inspect the repository first.",
        false,
        1,
    ));
}
