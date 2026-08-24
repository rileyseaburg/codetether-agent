use crate::approval::{session_command_grants, test_env::lock_env};
use crate::config::Config;
use serde_json::json;

struct GrantGuard;

impl Drop for GrantGuard {
    fn drop(&mut self) {
        session_command_grants::reset();
    }
}

#[test]
fn session_prefix_does_not_authorize_compound_shell_command() {
    let _lock = lock_env();
    session_command_grants::reset();
    let _guard = GrantGuard;
    session_command_grants::remember_scoped_request_in(
        "request",
        vec!["cargo test".into()],
        Some("session"),
        Some("workspace"),
    );
    session_command_grants::grant_for_request("request");
    let args = json!({
        "command": "cargo test --lib x; touch marker",
        "cwd": "workspace",
        "__ct_session_id": "session",
    });

    let blocked = crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(),
        "bash",
        &args,
    );

    assert!(
        blocked.is_some(),
        "compound command used a session prefix grant"
    );
}
