//! distinct outputs are preserved.
use super::*;

#[test]
pub(super) fn distinct_outputs_are_preserved() {
    let mut msgs = vec![
        tool_msg("a", &"x".repeat(1024)),
        tool_msg("b", &"y".repeat(1024)),
    ];
    for _ in 0..KEEP_LAST_MESSAGES {
        msgs.push(padding_msg());
    }
    let stats = dedup_tool_outputs(&mut msgs);
    assert_eq!(stats.dedup_hits, 0);
    assert_eq!(stats.total_bytes_saved, 0);
}
