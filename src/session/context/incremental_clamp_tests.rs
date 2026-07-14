//! Regression tests for active-turn-safe incremental overflow clamping.

use crate::provider::Role;
use crate::session::ResidencyLevel;

use super::incremental_clamp::clamp_and_recompute;
use super::incremental_clamp_test_fixtures::active_tool_turn;
use super::incremental_types::MessageOrigin;

#[test]
fn overflow_preserves_active_user_and_tool_pair() {
    let mut messages = active_tool_turn();
    let mut levels = vec![ResidencyLevel::Full; messages.len()];
    let mut origins = (0..messages.len()).map(MessageOrigin::Clone).collect();
    let (_, tags) = clamp_and_recompute(
        &mut messages,
        &mut levels,
        &mut origins,
        &"x".repeat(100),
        &[],
        1,
        4,
        &[],
    );
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[0].role, Role::User);
    assert_eq!(messages[1].role, Role::Assistant);
    assert_eq!(messages[2].role, Role::Tool);
    assert!(tags.contains(&"incremental_overflow_clamp"));
}
