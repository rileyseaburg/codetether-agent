//! short outputs are not deduplicated.
use super::*;

#[test]
pub(super) fn short_outputs_are_not_deduplicated() {
    let mut msgs = vec![tool_msg("a", "ok"), tool_msg("b", "ok")];
    for _ in 0..KEEP_LAST_MESSAGES {
        msgs.push(padding_msg());
    }
    let stats = dedup_tool_outputs(&mut msgs);
    assert_eq!(stats.dedup_hits, 0);
}
