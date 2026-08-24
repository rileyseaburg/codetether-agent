//! Concrete device access through an enforced command sandbox.

use super::command;
use crate::tool::bash_shell;
use crate::tool::sandbox::SandboxPolicy;

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn sandboxed_command_can_write_to_dev_null() {
    if let Some(reason) = crate::tool::sandbox::unavailable_reason() {
        panic!("mandatory sandbox unavailable: {reason}");
    }
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push("printf discarded >/dev/null && printf dev-null-ok".into());
    let cwd = std::env::current_dir().unwrap();
    let policy = SandboxPolicy {
        allowed_paths: vec![cwd.clone()],
        allow_exec: true,
        allow_network: true,
        ..SandboxPolicy::default()
    };
    let mut running = command(&shell.program, &args, &cwd, false, &[], Some(&policy))
        .await
        .unwrap();
    let poll = running.poll(1_000, 1_024).await.unwrap();
    assert_eq!(poll.exit_code, Some(0), "{}", poll.output);
    assert!(poll.output.contains("dev-null-ok"));
}