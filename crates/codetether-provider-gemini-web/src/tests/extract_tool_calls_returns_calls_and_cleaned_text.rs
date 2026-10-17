use super::*;

#[test]
fn extract_tool_calls_returns_calls_and_cleaned_text() {
    let text = r#"I will inspect the tree first.
<tool_call>
{"name":"tree","arguments":{"depth":3,"path":"."}}
</tool_call>
Then grep for key strings.
<tool_call>
{"name":"grep","arguments":{"pattern":"nextdoor","is_regex":false}}
</tool_call>"#;

    let (cleaned, calls) = GeminiWebProvider::extract_tool_calls(text);
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].0, "tree");
    assert!(calls[0].1.contains("\"depth\":3"));
    assert_eq!(calls[1].0, "grep");
    assert!(cleaned.contains("I will inspect the tree first."));
    assert!(cleaned.contains("Then grep for key strings."));
    assert!(!cleaned.contains("<tool_call>"));
}
