//! utf8 boundaries are respected.
use super::*;

#[test]
pub(super) fn utf8_boundaries_are_respected() {
    let emoji = "🦀".repeat(4000); // each crab = 4 bytes
    let mut msgs = vec![tool_msg(&emoji)];
    msgs.extend((0..RECENCY_WINDOW + 2).map(|i| user_msg(&format!("m{i}"))));
    let stats = snippet_stale_tool_outputs(&mut msgs);
    assert_eq!(stats.snippet_hits, 1);
    let ContentPart::ToolResult { content, .. } = &msgs[0].content[0] else {
        panic!();
    };
    // If we sliced mid-codepoint this would panic on re-render.
    assert!(content.is_char_boundary(0));
    assert!(content.contains("[...elided"));
}
