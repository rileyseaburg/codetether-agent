//! Tests for recall context part rendering.

use super::*;

#[test]
fn skips_thinking() {
    use crate::provider::ContentPart;
    let part = ContentPart::Thinking { text: "reasoning".into() };
    assert!(render_part(&part).is_empty());
}

#[test]
fn truncates_tool_result() {
    use crate::provider::ContentPart;
    let big = "x".repeat(5000);
    let part = ContentPart::ToolResult { tool_call_id: "c1".into(), content: big };
    assert!(render_part(&part).contains("[...truncated]"));
}

#[test]
fn truncate_bytes_char_boundary() {
    let s = "hello 🌍 world";
    let t = truncate_bytes(s, 10);
    assert!(t.len() <= 10);
}
