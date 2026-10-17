//! recent identical tool result is not deduplicated.
use super::*;

#[test]
pub(super) fn recent_identical_tool_result_is_not_deduplicated() {
    // Regression: when the user asks the agent to re-run a call,
    // the fresh output must remain verbatim. Previously this
    // replaced the retry's content with a back-reference, making
    // the retry appear to produce no new information.
    let big = "r".repeat(1024);
    let mut msgs = vec![tool_msg("old", &big), tool_msg("retry", &big)];
    let stats = dedup_tool_outputs(&mut msgs);
    assert_eq!(stats.dedup_hits, 0);
    let ContentPart::ToolResult { content, .. } = &msgs[1].content[0] else {
        panic!("expected tool result");
    };
    assert_eq!(content.len(), 1024);
}
