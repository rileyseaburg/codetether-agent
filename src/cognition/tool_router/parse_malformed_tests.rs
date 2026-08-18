//! Tests for tolerating malformed FunctionGemma output.

use super::parse::parse_functiongemma_response;

#[test]
fn parse_no_tool_calls() {
    assert!(parse_functiongemma_response("I cannot help with that request.").is_empty());
}

#[test]
fn parse_malformed_json_skipped() {
    let text = r#"<tool_call>
not valid json
</tool_call>
<tool_call>
{"name": "list_dir", "arguments": {"path": "."}}
</tool_call>"#;
    let calls = parse_functiongemma_response(text);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "list_dir");
}

#[test]
fn parse_empty_name_skipped() {
    let text = r#"<tool_call>
{"name": "", "arguments": {}}
</tool_call>"#;
    assert!(parse_functiongemma_response(text).is_empty());
}

#[test]
fn parse_unclosed_block_ignored() {
    assert!(parse_functiongemma_response("<tool_call>\n{\"name\": \"x\"}").is_empty());
}
