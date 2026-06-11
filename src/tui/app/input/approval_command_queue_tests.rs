use super::test_support as support;
use crate::approval::{ApprovalStatus, test_env::ENV_LOCK};
use crate::tui::app::state::{App, approval_queue};

#[test]
fn approve_session_uses_active_queued_request() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let (_data, _env, store, id) = support::setup();
    let mut app = App::default();

    assert!(super::run(&mut app, "/approve session"));

    assert_eq!(support::status(&store, &id), ApprovalStatus::Approved);
    assert!(crate::approval::session_grants::allowed(
        "bash", "execute", "bash:abc"
    ));
    assert!(approval_queue::active().is_none());
}

#[test]
fn abort_denies_active_queued_request() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let (_data, _env, store, id) = support::setup();
    let mut app = App::default();

    assert!(super::run(&mut app, "/abort"));

    assert_eq!(support::status(&store, &id), ApprovalStatus::Denied);
    assert!(approval_queue::active().is_none());
}

#[test]
fn approve_session_grants_remembered_command_prefix() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let (_data, _env, _store, id) = support::setup();
    crate::approval::session_command_grants::remember_request(&id, vec!["cargo test".into()]);
    let mut app = App::default();

    assert!(super::run(&mut app, "/approve session"));

    assert!(crate::approval::session_command_grants::allowed(
        "cargo test --lib tui"
    ));
}
