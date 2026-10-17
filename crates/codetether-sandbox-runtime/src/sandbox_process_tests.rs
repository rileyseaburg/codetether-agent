//! Command construction and waiting must not register setsid twice.

use std::collections::HashMap;

#[tokio::test]
async fn sandbox_wait_spawns_with_one_session_hook() {
    let root = tempfile::tempdir().unwrap();
    let args = vec![
        "-c".to_string(),
        "if (: >/dev/tty) 2>/dev/null; then exit 1; fi; printf session-isolated".to_string(),
    ];
    let (cmd, _) = super::super::sandbox_command::build(
        "/bin/sh",
        &args,
        root.path(),
        &HashMap::new(),
        None,
        0,
    );
    let mut violations = Vec::new();
    let output = super::wait(cmd, 5, &mut violations)
        .await
        .expect("a prepared command must not repeat setsid and fail with EPERM");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"session-isolated");
    assert!(violations.is_empty());
}
