use super::*;

#[test]
fn extract_tool_calls_strips_tool_result_blocks_when_calls_present() {
    let text = r#"<tool_call>{"name":"bash","arguments":{"command":"pwd"}}</tool_call>
<tool_result>{"bash":"fake"}</tool_result>"#;
    let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
    assert_eq!(calls.len(), 1);
    assert!(cleaned.is_empty());
}
