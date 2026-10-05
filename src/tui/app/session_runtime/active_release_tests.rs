//! Rejected submissions release only their own unstarted reservation.

use super::ActiveCancel;
use crate::session::helper::steering::SteeringInput;
use futures::FutureExt;
use std::sync::Arc;
use tokio::sync::Notify;

#[test]
fn answer_review_releases_prepared_steering_inbox() {
    let active = ActiveCancel::default();
    let id = uuid::Uuid::new_v4().to_string();
    assert!(active.prepare(&id));
    active.release_prepared(&id);
    assert!(!active.steer(SteeringInput::new("late".into(), vec![])));
    assert!(active.prepare(&id));
    active.clear();
}

#[test]
fn answer_review_rejection_preserves_other_reservation_and_running_turn() {
    let active = ActiveCancel::default();
    let id = uuid::Uuid::new_v4().to_string();
    assert!(active.prepare(&id));
    active.release_prepared("different-session");
    assert!(!active.prepare(&id));
    let notify = Arc::new(Notify::new());
    assert!(active.set(&id, Arc::clone(&notify)));
    active.release_prepared(&id);
    assert!(!active.prepare(&id));
    assert!(active.notify());
    assert!(notify.notified().now_or_never().is_some());
    active.clear();
}

#[test]
fn answer_review_rejection_without_reservation_is_harmless() {
    let active = ActiveCancel::default();
    active.release_prepared("idle-session");
    assert!(!active.notify());
}
