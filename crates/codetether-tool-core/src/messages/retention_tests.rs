use super::tool_result_with_status;
use codetether_provider_core::ContentPart;

#[test]
fn preserves_large_tool_output_in_session_history() {
    let output = format!("{}tail: 日本語 🦀", "x".repeat(5000));
    let msg = tool_result_with_status("call-1".into(), "bash", true, output.clone());
    let ContentPart::ToolResult { content, .. } = &msg.content[0] else {
        panic!("expected tool result");
    };
    assert!(content.contains(&output));
    assert!(!content.contains("runtime digest"));
    assert!(content.ends_with("tail: 日本語 🦀"));
    assert!(content.contains("- status: success"));
}
