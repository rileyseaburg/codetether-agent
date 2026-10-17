//! bucket for turn maps correctly policy.
use super::*;

#[test]
pub(super) fn bucket_for_turn_maps_correctly() {
    let b = bucket_for_turn("hard", true);
    assert!(matches!(b.difficulty, Difficulty::Hard));
    assert!(matches!(b.tool_use, ToolUse::Yes));
}
