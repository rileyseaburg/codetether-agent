use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::approval::{ApprovalStatus, ApprovalStore, LiveApprovalRequest, test_env::lock_env};
use crate::tui::app::event_handlers::keyboard::handle_ctrl_key;
use crate::tui::app::session_runtime;
use crate::tui::app::state::{App, approval_queue};

struct EnvGuard;

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
        approval_queue::reset();
    }
}

fn setup() -> (tempfile::TempDir, EnvGuard, ApprovalStore, String) {
    approval_queue::reset();
    let dir = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", dir.path()) };
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:pwd", "test")
        .unwrap();
    approval_queue::push(LiveApprovalRequest::new(
        request.id.clone(),
        "call-1".into(),
        "bash".into(),
        "execute".into(),
        "bash:pwd".into(),
        "test".into(),
    ));
    (dir, EnvGuard, store, request.id)
}

#[tokio::test(flavor = "current_thread")]
async fn ctrl_a_approves_active_request() {
    let _lock = lock_env();
    let (_dir, _env, store, id) = setup();
    let mut app = App::default();
    let (tx, _) = tokio::sync::mpsc::channel(1);
    let runtime = session_runtime::spawn(tx, tokio::sync::mpsc::channel(1).0);

    let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
    let handled = handle_ctrl_key(&mut app, ".".as_ref(), &runtime, key)
        .unwrap()
        .unwrap();

    assert!(!handled);
    assert_eq!(
        store.decision(&id).unwrap().unwrap().status,
        ApprovalStatus::Approved
    );
    assert!(approval_queue::active().is_none());
}
