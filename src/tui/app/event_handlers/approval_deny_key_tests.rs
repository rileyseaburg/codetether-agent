//! Ctrl+D persists denial for the active approval request.

#[path = "approval_deny_key_test_env.rs"]
mod env;

use crate::approval::{ApprovalStatus, ApprovalStore, LiveApprovalDecision, test_env::lock_env};
use crate::tui::app::{
    session_runtime,
    state::{App, approval_queue},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[tokio::test(flavor = "current_thread")]
async fn ctrl_d_denies_active_request() {
    let _lock = lock_env();
    approval_queue::reset();
    let dir = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", dir.path()) };
    let _env = env::Guard;
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:pwd", "test")
        .unwrap();
    let (waiter, mut events) = env::waiter(request.id.clone()).await;
    let mut app = App::default();
    let (tx, _) = tokio::sync::mpsc::channel(1);
    let runtime = session_runtime::spawn(tx, tokio::sync::mpsc::channel(1).0);

    let key = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL);
    let handled = super::super::keyboard::handle_ctrl_key(&mut app, ".".as_ref(), &runtime, key)
        .unwrap()
        .unwrap();
    assert!(!handled);
    assert_eq!(
        store.decision(&request.id).unwrap().unwrap().status,
        ApprovalStatus::Denied
    );
    assert!(approval_queue::active().is_none());
    assert!(matches!(
        waiter.await.unwrap(),
        LiveApprovalDecision::Denied { .. }
    ));
    let crate::session::SessionEvent::ToolCallMetadata { metadata, .. } =
        events.recv().await.unwrap()
    else {
        panic!("decision metadata")
    };
    assert_eq!(metadata["approval_decision"]["status"], "denied");
}
