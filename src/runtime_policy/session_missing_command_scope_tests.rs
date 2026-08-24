use crate::approval::{session_command_grants, test_env::lock_env};

struct GrantGuard;

impl Drop for GrantGuard {
    fn drop(&mut self) {
        session_command_grants::reset();
    }
}

#[test]
fn missing_session_id_never_creates_command_grant() {
    let _lock = lock_env();
    session_command_grants::reset();
    let _guard = GrantGuard;
    session_command_grants::remember_scoped_request_in(
        "request",
        vec!["cargo".into()],
        None,
        Some("workspace"),
    );
    session_command_grants::grant_for_request("request");
    assert!(!session_command_grants::allowed_scoped_in(
        "cargo test",
        None,
        Some("workspace"),
    ));
    session_command_grants::remember_scoped_request_in(
        "request",
        vec!["cargo".into()],
        Some("   "),
        Some("workspace"),
    );
    session_command_grants::grant_for_request("request");
    assert!(!session_command_grants::allowed_scoped_in(
        "cargo test",
        Some("   "),
        Some("workspace"),
    ));
}
