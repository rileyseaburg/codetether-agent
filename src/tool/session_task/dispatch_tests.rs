#[test]
fn accepts_legacy_complete_goal_action() {
    assert_eq!(super::canonical("complete_goal"), "update_goal_complete");
}
