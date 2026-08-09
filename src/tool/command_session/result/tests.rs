use super::tool_result;
use crate::tool::command_session::{Poll, SpawnMetadata};

#[test]
fn redacts_command_credentials_before_returning_output() {
    let metadata = SpawnMetadata {
        sandboxed: false,
        interactive: false,
        redactions: vec!["secret-token".into()],
        unsafe_fallbacks: Vec::new(),
        cwd: std::env::current_dir().unwrap(),
    };
    let poll = Poll {
        output: "token=secret-token".into(),
        running: false,
        exit_code: Some(0),
        elapsed: std::time::Duration::ZERO,
        omitted_bytes: 0,
        poll_bytes: 17,
        silent_for: std::time::Duration::ZERO,
        session_bytes: 17,
    };
    let result = tool_result(poll, &metadata, None);
    assert!(result.output.contains("[REDACTED]"));
    assert!(!result.output.contains("secret-token"));
}
