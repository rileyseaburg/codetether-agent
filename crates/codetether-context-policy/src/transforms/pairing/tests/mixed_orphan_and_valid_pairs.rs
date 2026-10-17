//! Existing pairing regression.
use super::*;

#[test]
fn mixed_orphan_and_valid_pairs() {
    let mut msgs = vec![
        user("q"),
        call("a"), // will be orphaned
        call("b"),
        result("b"),
        user("mid"),
        result("c"), // orphan — no prior call_c
    ];
    let stats = repair_orphans(&mut msgs);
    // One synthetic injection for "a", one drop for "c".
    assert_eq!(stats.snippet_hits, 2);
    // a's synthetic result should be inserted right after call("a").
    assert_eq!(msgs[1].role, Role::Assistant);
    assert_eq!(msgs[2].role, Role::Tool);
    let ContentPart::ToolResult { tool_call_id, .. } = &msgs[2].content[0] else {
        panic!();
    };
    assert_eq!(tool_call_id, "a");
}
