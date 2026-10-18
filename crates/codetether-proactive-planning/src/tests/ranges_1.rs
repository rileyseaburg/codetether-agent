use crate::planned;

#[test]
fn long_sessions_keep_plans_bounded() {
    assert!(planned(10_000).len() <= 97);
}
