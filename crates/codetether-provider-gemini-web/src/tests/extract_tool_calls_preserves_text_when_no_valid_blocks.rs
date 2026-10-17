use super::*;

#[test]
fn extract_tool_calls_preserves_text_when_no_valid_blocks() {
    let text = "<tool_call>{not-json}</tool_call>";
    let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
    assert!(calls.is_empty());
    assert_eq!(cleaned, text);
}
