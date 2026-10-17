//! Child observation never claims ownership or changes replacement generations.

use crate::ControlPlane;
use futures::FutureExt;

#[test]
fn observers_share_pause_and_cancel_without_claiming_executor() {
    let control = ControlPlane::default();
    let lease = control.reserve("parent").unwrap();
    let mut first = control.subscribe(&lease).unwrap();
    let mut second = control.subscribe(&lease).unwrap();
    assert!(control.begin("parent").is_some());
    assert!(control.pause(&lease));
    assert!(control.is_paused(&lease));
    assert!(first.cancelled().now_or_never().is_none());
    assert!(control.resume(&lease));
    assert!(!control.is_paused(&lease));
    assert!(control.cancel(&lease));
    assert!(first.cancelled().now_or_never().is_some());
    assert!(second.cancelled().now_or_never().is_some());
}

#[test]
fn release_wakes_children_and_stale_controls_cannot_pause_replacement() {
    let control = ControlPlane::default();
    let lease = control.reserve("parent").unwrap();
    let mut child = control.subscribe(&lease).unwrap();
    assert!(control.finish(&lease));
    assert!(child.cancelled().now_or_never().is_some());
    let next = control.reserve("parent").unwrap();
    assert!(!control.pause(&lease));
    assert!(!control.resume(&lease));
    assert!(!control.is_paused(&next));
    assert!(control.is_stopped(&lease));
    assert!(!control.is_stopped(&next));
    assert!(control.subscribe(&lease).is_none());
}
