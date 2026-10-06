//! A live human goal change cannot lose cancellation before executor attachment.

use super::super::ActiveTurn;
use futures::FutureExt;
use std::sync::Arc;
use tokio::sync::Notify;

#[test]
fn goal_control_cancel_before_start_is_delivered_once() {
    let mut turn = ActiveTurn::default();
    assert!(turn.prepare("goal-session"));
    assert!(turn.request_cancel());
    let cancel = Arc::new(Notify::new());
    assert!(turn.attach("goal-session", Arc::clone(&cancel)));
    assert!(cancel.notified().now_or_never().is_some());
    assert!(cancel.notified().now_or_never().is_none());
}

#[test]
fn goal_control_old_cancel_does_not_cancel_the_next_turn() {
    let mut turn = ActiveTurn::default();
    assert!(turn.prepare("first"));
    assert!(turn.request_cancel());
    turn.clear();
    assert!(!turn.request_cancel());
    let cancel = Arc::new(Notify::new());
    assert!(turn.attach("second", Arc::clone(&cancel)));
    assert!(cancel.notified().now_or_never().is_none());
}
