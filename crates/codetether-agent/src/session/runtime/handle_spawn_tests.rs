//! Inherit the parent lease across direct-tool task spawning.

use super::{ControlHandle, RunGuard};
use futures::FutureExt;

#[tokio::test]
async fn guarded_execution_propagates_control_into_spawned_tools() {
    let id = uuid::Uuid::new_v4().to_string();
    let mut guard = RunGuard::open(&id).unwrap();
    let child_id = guard
        .run(async {
            let child = ControlHandle::spawn(async {
                ControlHandle::current()
                    .unwrap()
                    .session_id()
                    .unwrap()
                    .to_owned()
            });
            Ok(child.await.unwrap())
        })
        .await
        .unwrap();
    assert_eq!(child_id, id);
    assert!(ControlHandle::current().is_none());
}

#[tokio::test]
async fn transient_pause_holds_readiness_until_resume() {
    let handle = ControlHandle::new();
    handle.pause();
    assert!(handle.ready().now_or_never().is_none());
    handle.resume();
    handle.ready().await.unwrap();
}
