//! Cancellation and identity invariants across executor handoff.

use crate::ActiveTurn;
use futures::FutureExt;
use std::sync::Arc;
use tokio::sync::Notify;

#[test]
fn cancellation_survives_handoff_and_is_delivered_once() {
    let mut turn = ActiveTurn::default();
    assert!(!turn.request_cancel());
    assert!(turn.prepare("first"));
    assert!(turn.request_cancel());
    assert!(turn.request_cancel());
    let notify = Arc::new(Notify::new());
    assert!(!turn.attach("other", Arc::clone(&notify)));
    assert!(turn.attach("first", Arc::clone(&notify)));
    assert!(notify.notified().now_or_never().is_some());
    assert!(notify.notified().now_or_never().is_none());
    assert!(!turn.attach("first", Arc::clone(&notify)));
}

#[test]
fn clearing_prevents_stale_cancellation_from_reaching_next_turn() {
    let mut turn = ActiveTurn::default();
    assert!(turn.prepare("first"));
    assert!(!turn.prepare("second"));
    assert!(turn.request_cancel());
    assert_eq!(turn.clear().as_deref(), Some("first"));
    assert!(!turn.request_cancel());
    let notify = Arc::new(Notify::new());
    assert!(turn.attach("second", Arc::clone(&notify)));
    assert!(notify.notified().now_or_never().is_none());
    assert!(turn.request_cancel());
    assert!(notify.notified().now_or_never().is_some());
    assert_eq!(turn.session_id(), Some("second"));
}
