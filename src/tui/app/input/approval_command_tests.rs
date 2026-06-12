use super::approval_command;
use crate::approval::{ApprovalStatus, ApprovalStore, test_env::lock_env};
use crate::tui::app::state::App;

struct EnvGuard;

impl EnvGuard {
    fn data_dir(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[test]
fn approve_records_decision_without_retry_prompt() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:abc", "runtime policy")
        .expect("request");
    let mut app = App::default();

    assert!(approval_command::run(
        &mut app,
        &format!("/approve {}", request.id)
    ));

    let decision = store.decision(&request.id).expect("decision").unwrap();
    assert_eq!(decision.status, ApprovalStatus::Approved);
    assert!(app.state.input.is_empty());
    assert!(app.state.status.contains("recorded"));
}
