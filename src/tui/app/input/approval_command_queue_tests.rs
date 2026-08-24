use super::test_support as support;
use crate::approval::{ApprovalDecisionKind, ApprovalStatus, test_env::lock_env};
use crate::tui::app::state::{App, approval_queue};

#[test]
fn approve_session_uses_active_queued_request() {
    let _lock = lock_env();
    let (_data, _env, store, id) = support::setup();
    let mut app = App::default();

    assert!(super::run(&mut app, "/approve session"));

    assert_eq!(support::status(&store, &id), ApprovalStatus::Approved);
    assert!(crate::approval::session_grants::allowed_scoped(
        "bash",
        "execute",
        "bash:abc",
        Some(support::SESSION)
    ));
    assert!(approval_queue::active().is_none());
    assert_eq!(
        store.decision(&id).expect("decision").expect("record").kind,
        Some(ApprovalDecisionKind::ApproveForSession)
    );
}

#[test]
fn abort_denies_active_queued_request() {
    let _lock = lock_env();
    let (_data, _env, store, id) = support::setup();
    let mut app = App::default();

    assert!(super::run(&mut app, "/abort"));

    assert_eq!(support::status(&store, &id), ApprovalStatus::Denied);
    assert!(approval_queue::active().is_none());
}

#[test]
fn approve_session_grants_remembered_command_prefix() {
    let _lock = lock_env();
    let (_data, _env, _store, id) = support::setup();
    crate::approval::session_command_grants::remember_scoped_request_in(
        &id,
        vec!["cargo test".into()],
        Some(support::SESSION),
        Some("tui-workspace"),
    );
    let mut app = App::default();

    assert!(super::run(&mut app, "/approve session"));

    assert!(crate::approval::session_command_grants::allowed_scoped_in(
        "cargo test --lib tui",
        Some(support::SESSION),
        Some("tui-workspace")
    ));
}
