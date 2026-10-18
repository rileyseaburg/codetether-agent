use crate::approval::{ApprovalStatus, ApprovalStore, LiveApprovalRequest};
use crate::tui::app::state::approval_queue;

pub(super) struct EnvGuard;

impl EnvGuard {
    pub(super) fn new(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        approval_queue::reset();
        crate::approval::session_grants::reset();
        crate::approval::session_command_grants::reset();
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        approval_queue::reset();
        crate::approval::session_grants::reset();
        crate::approval::session_command_grants::reset();
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

pub(super) fn setup() -> (tempfile::TempDir, EnvGuard, ApprovalStore, String) {
    let data = tempfile::tempdir().expect("tempdir");
    let env = EnvGuard::new(data.path());
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:abc", "runtime policy")
        .expect("request");
    queue(&request.id);
    (data, env, store, request.id)
}

pub(super) fn status(store: &ApprovalStore, id: &str) -> ApprovalStatus {
    store
        .decision(id)
        .expect("decision")
        .expect("recorded decision")
        .status
}

fn queue(id: &str) {
    approval_queue::push(LiveApprovalRequest::new(
        id.to_string(),
        "call".to_string(),
        "bash".to_string(),
        "execute".to_string(),
        "bash:abc".to_string(),
        "runtime policy".to_string(),
    ));
}
