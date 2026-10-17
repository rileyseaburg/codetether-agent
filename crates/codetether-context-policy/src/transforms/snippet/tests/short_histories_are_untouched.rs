//! short histories are untouched.
use super::*;

#[test]
pub(super) fn short_histories_are_untouched() {
    let mut msgs = vec![tool_msg(&"x".repeat(10_000))];
    let stats = snippet_stale_tool_outputs(&mut msgs);
    assert_eq!(stats.snippet_hits, 0);
}
