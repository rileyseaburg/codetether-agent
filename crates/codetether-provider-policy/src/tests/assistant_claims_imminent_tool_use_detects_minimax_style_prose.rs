//! Original session provider-policy regression case.
use crate::assistant_claims_imminent_tool_use;
use codetether_provider_core::ToolDefinition;
use serde_json::json;

#[test]
fn assistant_claims_imminent_tool_use_detects_minimax_style_prose() {
    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "shell".to_string(),
        parameters: json!({"type":"object"}),
    }];

    assert!(assistant_claims_imminent_tool_use(
        "First, I'll use bash to inspect the repo and then patch the issue.",
        &tools,
    ));
    assert!(!assistant_claims_imminent_tool_use(
        "The fix is to update the parser and rerun tests.",
        &tools,
    ));
}
