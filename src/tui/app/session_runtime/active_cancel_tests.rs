use futures::FutureExt;

use super::*;

#[test]
fn notifies_only_while_a_turn_is_active() {
    let active = ActiveCancel::default();
    let notify = Arc::new(Notify::new());
    assert!(!active.notify());
    assert!(active.set("session", Arc::clone(&notify)));
    assert!(active.notify());
    assert!(notify.notified().now_or_never().is_some());
    active.clear();
    assert!(!active.notify());
}

#[test]
fn steering_is_scoped_to_prepared_turn() {
    let active = ActiveCancel::default();
    let input = || SteeringInput::new("adjust".into(), vec![]);
    assert!(!active.steer(input()));
    assert!(active.prepare("session"));
    assert!(active.steer(input()));
    active.clear();
    assert!(!active.steer(input()));
}
