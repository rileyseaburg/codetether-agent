use crate::approval::{ApprovalStore, session_grants, test_env::lock_env};

struct GrantGuard;

impl Drop for GrantGuard {
    fn drop(&mut self) {
        session_grants::reset();
    }
}

#[test]
fn missing_session_id_never_creates_global_grant() {
    let _lock = lock_env();
    session_grants::reset();
    let _guard = GrantGuard;
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let request = store
        .create_request("bash", "execute", "bash:abc", "test")
        .expect("request");
    session_grants::remember_request(&request.id, None);
    let receipt = store.approve(&request.id, "test", "ok").expect("approve");
    session_grants::grant(&receipt);

    assert!(!session_grants::allowed_scoped(
        "bash", "execute", "bash:abc", None
    ));
    session_grants::remember_request(&request.id, Some("   "));
    session_grants::grant(&receipt);
    assert!(!session_grants::allowed_scoped(
        "bash",
        "execute",
        "bash:abc",
        Some("   ")
    ));
}
