use super::*;

#[test]
fn extract_tool_calls_accepts_escaped_tool_markup() {
    let text = r#"I will invoke LSP now.
\<tool\_call\>
{"name":"lsp","arguments":{"action":"hover","file\_path":"api/src/a.ts","line":1,"column":1}}
\</tool\_call\>"#;

    let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "lsp");
    assert!(calls[0].1.contains("\"file_path\":\"api/src/a.ts\""));
    assert!(cleaned.contains("I will invoke LSP now."));
    assert!(!cleaned.contains("tool_call"));
}
