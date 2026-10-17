//! One ownership table serves frontend handoff and executor cancellation.

use crate::ControlPlane;
use futures::FutureExt;
use std::sync::Arc;
use tokio::sync::Notify;

#[test]
fn cancellation_survives_reservation_and_reaches_both_subscribers() {
    let control = ControlPlane::default();
    let lease = control.reserve("session").unwrap();
    assert!(control.cancel(&lease));
    let notify = Arc::new(Notify::new());
    assert!(control.attach(&lease, Arc::clone(&notify)));
    assert!(notify.notified().now_or_never().is_some());
    let mut execution = control.begin("session").unwrap();
    assert!(execution.cancelled().now_or_never().is_some());
    assert!(!control.release_prepared(&lease));
    assert!(control.finish(&lease));
}

#[test]
fn ownership_is_exclusive_per_session_and_cleanup_is_generation_checked() {
    let control = ControlPlane::default();
    let first = control.begin("session").unwrap();
    assert!(control.begin("session").is_none());
    assert!(control.reserve("session").is_none());
    assert!(control.begin("independent").is_some());
    assert!(control.finish(first.lease()));
    let mut second = control.begin("session").unwrap();
    assert!(!control.cancel(first.lease()));
    assert!(!control.finish(first.lease()));
    assert!(!control.is_current(first.lease()));
    assert!(second.cancelled().now_or_never().is_none());
    assert!(control.cancel_session("session"));
    assert!(second.cancelled().now_or_never().is_some());
}

#[test]
fn rejected_handoff_releases_only_an_unstarted_reservation() {
    let control = ControlPlane::default();
    let lease = control.reserve("session").unwrap();
    assert!(control.release_prepared(&lease));
    let execution = control.begin("session").unwrap();
    assert!(!control.release_prepared(execution.lease()));
    assert!(!control.release_prepared(&lease));
    assert!(!control.cancel_session("idle"));
}
