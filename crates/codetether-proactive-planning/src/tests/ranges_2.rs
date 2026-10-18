use crate::planned;

#[test]
fn short_sessions_do_not_schedule_noise() {
    assert!(planned(11).is_empty());
}
