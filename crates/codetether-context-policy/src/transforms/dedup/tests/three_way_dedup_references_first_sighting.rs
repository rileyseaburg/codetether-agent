//! three way dedup references first sighting.
use super::*;

#[test]
pub(super) fn three_way_dedup_references_first_sighting() {
    let big = "z".repeat(2048);
    let mut msgs = vec![
        tool_msg("first", &big),
        tool_msg("second", &big),
        tool_msg("third", &big),
    ];
    for _ in 0..KEEP_LAST_MESSAGES {
        msgs.push(padding_msg());
    }
    let stats = dedup_tool_outputs(&mut msgs);
    assert_eq!(stats.dedup_hits, 2);

    for idx in [1, 2] {
        let ContentPart::ToolResult { content, .. } = &msgs[idx].content[0] else {
            panic!("expected tool result");
        };
        assert!(content.contains("tool_call_id=first"));
    }
}
