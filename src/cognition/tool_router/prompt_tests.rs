//! Tests for FunctionGemma prompt construction.

use super::prompt::build_functiongemma_prompt;
use crate::provider::ToolDefinition;

fn read_file_tool() -> ToolDefinition {
    ToolDefinition {
        name: "read_file".to_string(),
        description: "Read a file".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": { "path": { "type": "string" } },
            "required": ["path"]
        }),
    }
}

#[test]
fn prompt_contains_tool_definitions() {
    let prompt = build_functiongemma_prompt("Please read foo.rs", &[read_file_tool()]);
    assert!(prompt.contains("<start_of_turn>system"));
    assert!(prompt.contains("read_file"));
    assert!(prompt.contains("<tools>"));
    assert!(prompt.contains("Please read foo.rs"));
    assert!(prompt.contains("<start_of_turn>model"));
}

#[test]
fn prompt_truncates_long_text_on_char_boundary() {
    // Multi-byte characters must not be split mid-sequence.
    let long_text = "é".repeat(600);
    let prompt = build_functiongemma_prompt(&long_text, &[read_file_tool()]);
    assert!(prompt.contains("<start_of_turn>model"));
}
