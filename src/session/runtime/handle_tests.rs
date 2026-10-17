//! Ownership and explicit propagation for delegated work.

use super::ControlHandle;
use crate::session::runtime::{CONTROL, RunGuard};

#[tokio::test]
async fn last_standalone_clone_releases_run_and_attached_drop_preserves_owner() {
    let handle = ControlHandle::new();
    let lease = handle.0.lease.clone();
    let clone = handle.clone();
    drop(handle);
    assert!(CONTROL.is_current(&lease));
    drop(clone);
    assert!(!CONTROL.is_current(&lease));
    let id = uuid::Uuid::new_v4().to_string();
    let guard = RunGuard::open(&id).unwrap();
    let attached = guard.handle();
    drop(attached.clone());
    assert!(RunGuard::open(&id).is_err());
    drop(guard);
    assert!(attached.is_cancelled());
}

#[tokio::test]
async fn scope_and_independent_cursors_broadcast_every_input() {
    let handle = ControlHandle::new();
    handle
        .scope(async {
            let child = ControlHandle::current().unwrap();
            handle.publish_input(crate::provider::Message {
                role: crate::provider::Role::User,
                content: vec![],
            });
            let (mut first, mut second) = (0, 0);
            assert_eq!(child.inputs_since(&mut first).len(), 1);
            assert_eq!(handle.inputs_since(&mut second).len(), 1);
            assert!(child.inputs_since(&mut first).is_empty());
            handle.pause();
            handle.cancel();
            assert!(child.ready().await.is_err());
            child.cancelled().await;
        })
        .await;
    assert!(ControlHandle::current().is_none());
}
