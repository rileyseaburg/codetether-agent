//! Existing pairing regression.
use super::*;

#[test]
fn orphan_call_gets_synthetic_result() {
    let mut msgs = vec![user("q"), call("a"), user("follow-up")];
    repair_orphans(&mut msgs);
    assert_eq!(msgs.len(), 4);
    assert_eq!(msgs[2].role, Role::Tool);
    let ContentPart::ToolResult {
        tool_call_id,
        content,
    } = &msgs[2].content[0]
    else {
        panic!("expected synthetic tool result");
    };
    assert_eq!(tool_call_id, "a");
    assert!(content.contains("unavailable"));
}
