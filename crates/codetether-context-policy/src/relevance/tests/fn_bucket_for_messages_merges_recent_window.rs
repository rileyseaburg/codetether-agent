//! bucket for messages merges recent window policy.
use super::*;

#[test]
pub(super) fn bucket_for_messages_merges_recent_window() {
    let bucket = bucket_for_messages(&[
        text("edited src/lib.rs"),
        tool_call("Shell"),
        tool_result("Error: broken"),
    ]);
    assert_eq!(bucket.tool_use, ToolUse::Yes);
    assert_eq!(bucket.dependency, Dependency::Chained);
    assert_eq!(bucket.difficulty, Difficulty::Medium);
}
