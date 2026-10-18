//! Shared registered-tool fixture for recovery tests.
use codetether_provider_core::ToolDefinition;

pub(super) fn bash_tool() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "bash".to_string(),
        description: "shell".to_string(),
        parameters: serde_json::json!({"type": "object"}),
    }]
}
