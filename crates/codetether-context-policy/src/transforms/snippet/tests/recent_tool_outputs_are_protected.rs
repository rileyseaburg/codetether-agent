//! recent tool outputs are protected.
use super::*;

#[test]
pub(super) fn recent_tool_outputs_are_protected() {
    let mut msgs: Vec<Message> = (0..RECENCY_WINDOW)
        .map(|i| user_msg(&format!("m{i}")))
        .collect();
    msgs.insert(0, tool_msg(&"x".repeat(10_000)));
    let stats = snippet_stale_tool_outputs(&mut msgs);
    // The oversized tool_msg is at index 0 and total is RECENCY_WINDOW+1,
    // so stale_upto=1 → it IS eligible. Confirm that.
    assert_eq!(stats.snippet_hits, 1);
}
