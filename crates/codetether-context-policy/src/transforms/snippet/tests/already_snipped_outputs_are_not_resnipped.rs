//! already snipped outputs are not resnipped.
use super::*;

#[test]
pub(super) fn already_snipped_outputs_are_not_resnipped() {
    let mut msgs = vec![tool_msg(&format!(
        "{}\n[...elided 99 bytes...]\n{}",
        "H".repeat(2000),
        "T".repeat(2000)
    ))];
    msgs.extend((0..RECENCY_WINDOW + 2).map(|i| user_msg(&format!("m{i}"))));
    let before_len = match &msgs[0].content[0] {
        ContentPart::ToolResult { content, .. } => content.len(),
        _ => unreachable!(),
    };
    let stats = snippet_stale_tool_outputs(&mut msgs);
    assert_eq!(stats.snippet_hits, 0);
    let after_len = match &msgs[0].content[0] {
        ContentPart::ToolResult { content, .. } => content.len(),
        _ => unreachable!(),
    };
    assert_eq!(before_len, after_len);
}
