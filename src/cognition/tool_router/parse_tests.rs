//! Tests for well-formed FunctionGemma response parsing.

use super::parse::parse_functiongemma_response;

#[test]
fn parse_single_tool_call() {
    let text = r#"<tool_call>
{"name": "read_file", "arguments": {"path": "/tmp/foo.rs"}}
</tool_call>"#;
    let calls = parse_functiongemma_response(text);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "read_file");
    assert!(calls[0].arguments.contains("/tmp/foo.rs"));
}

#[test]
fn parse_multiple_tool_calls() {
    let text = r#"I'll read both files.
<tool_call>
{"name": "read_file", "arguments": {"path": "a.rs"}}
</tool_call>
<tool_call>
{"name": "read_file", "arguments": {"path": "b.rs"}}
</tool_call>"#;
    let calls = parse_functiongemma_response(text);
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].name, "read_file");
    assert_eq!(calls[1].name, "read_file");
}
