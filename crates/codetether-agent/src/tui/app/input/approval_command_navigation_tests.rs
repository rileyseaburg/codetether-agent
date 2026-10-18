//! Browsing keeps implicit approval decisions bound to visible content.

use super::order_support::{queue, request};
use super::test_support::EnvGuard;
use crate::approval::{LiveApprovalDecision, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};

#[tokio::test]
async fn approval_decision_targets_browsed_request_not_original_head() {
    let _lock = lock_env();
    let data = tempfile::tempdir().unwrap();
    let _env = EnvGuard::new(data.path());
    let store = crate::approval::ApprovalStore::open_default().unwrap();
    let first = store
        .create_request("bash", "execute", "one", "test")
        .unwrap();
    let second = store
        .create_request("bash", "execute", "two", "test")
        .unwrap();
    let (tx, mut rx) = tokio::sync::mpsc::channel(2);
    let first_waiter = queue(&tx, &mut rx, request(&first.id, "one")).await;
    let second_waiter = queue(&tx, &mut rx, request(&second.id, "two")).await;
    approval_queue::cycle(true);
    assert_eq!(
        approval_queue::active_id().as_deref(),
        Some(second.id.as_str())
    );
    let mut app = App::default();
    assert!(super::run(&mut app, "/approve"));
    assert_eq!(second_waiter.await.unwrap(), LiveApprovalDecision::Approved);
    assert!(store.decision(&first.id).unwrap().is_none());
    assert_eq!(
        approval_queue::active_id().as_deref(),
        Some(first.id.as_str())
    );
    crate::approval::live::decide(&first.id, LiveApprovalDecision::denied());
    approval_queue::resolve(&first.id);
    assert_eq!(first_waiter.await.unwrap(), LiveApprovalDecision::denied());
}
