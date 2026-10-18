//! Regression tests for implicit approval target ordering.

use super::order_support::{queue, request};
use super::test_support::EnvGuard;
use crate::approval::{LiveApprovalDecision, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};

#[tokio::test]
async fn implicit_decision_targets_visible_oldest_request() {
    let _lock = lock_env();
    let data = tempfile::tempdir().unwrap();
    let _env = EnvGuard::new(data.path());
    let store = crate::approval::ApprovalStore::open_default().unwrap();
    let create = |resource| {
        store
            .create_request("bash", "execute", resource, "test")
            .unwrap()
    };
    let first = create("bash:one");
    let second = create("bash:two");
    let (tx, mut rx) = tokio::sync::mpsc::channel(2);
    let first_waiter = queue(&tx, &mut rx, request(&first.id, "one")).await;
    let second_waiter = queue(&tx, &mut rx, request(&second.id, "two")).await;

    let mut app = App::default();
    app.state.approval_preview_scroll = 42;
    assert!(super::run(&mut app, "/approve"));
    assert_eq!(approval_queue::active_id().as_deref(), Some(&*second.id));
    assert!(
        store
            .decision(&first.id)
            .unwrap()
            .unwrap()
            .status
            .is_approved()
    );
    assert!(store.decision(&second.id).unwrap().is_none());
    assert_eq!(first_waiter.await.unwrap(), LiveApprovalDecision::Approved);
    assert_eq!(app.state.approval_preview_scroll, 0);
    assert!(app.state.status.starts_with("Approved once"));
    assert!(app.state.status.contains(&second.id));
    crate::approval::live::decide(&second.id, LiveApprovalDecision::denied());
    approval_queue::resolve(&second.id);
    assert_eq!(second_waiter.await.unwrap(), LiveApprovalDecision::denied());
}
