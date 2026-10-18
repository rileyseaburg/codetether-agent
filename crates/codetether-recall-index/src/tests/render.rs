//! Tests for recall context part rendering.

use crate::{part::render_part, truncate::truncate_bytes};

#[test]
fn skips_thinking() {
    use codetether_provider_core::ContentPart;
    let part = ContentPart::Thinking {
        text: "reasoning".into(),
        signature: None,
    };
    assert!(render_part(&part).is_empty());
}

#[test]
fn truncates_tool_result() {
    use codetether_provider_core::ContentPart;
    let big = "x".repeat(5000);
    let part = ContentPart::ToolResult {
        tool_call_id: "c1".into(),
        content: big,
    };
    assert!(render_part(&part).contains("[...truncated]"));
}

#[test]
fn truncate_bytes_char_boundary() {
    let s = "hello 🌍 world";
    let t = truncate_bytes(s, 10);
    assert!(t.len() <= 10);
}
