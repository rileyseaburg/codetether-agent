use super::command;
use crate::tool::bash_shell;
use crate::tool::sandbox::SandboxPolicy;

#[tokio::test]
async fn sandboxed_interactive_command_keeps_its_pty() {
    if crate::tool::sandbox::unavailable_reason().is_some() {
        return;
    }
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push("test -t 0 && test -t 1 && printf sandbox-tty".into());
    let cwd = std::env::current_dir().unwrap();
    let policy = SandboxPolicy {
        allowed_paths: vec![cwd.clone()],
        allow_exec: true,
        ..SandboxPolicy::default()
    };
    let mut running = command(&shell.program, &args, &cwd, true, &[], Some(&policy))
        .await
        .unwrap();
    let poll = running.poll(1_000, 1_024).await.unwrap();
    assert!(!poll.running);
    assert_eq!(poll.exit_code, Some(0));
    assert!(poll.output.contains("sandbox-tty"));
    assert!(running.metadata.sandboxed);
}

#[tokio::test]
async fn sandboxed_command_can_write_to_dev_null() {
    if crate::tool::sandbox::unavailable_reason().is_some() {
        return;
    }
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push("printf discarded >/dev/null && printf dev-null-ok".into());
    let cwd = std::env::current_dir().unwrap();
    let policy = SandboxPolicy {
        allowed_paths: vec![cwd.clone()],
        allow_exec: true,
        ..SandboxPolicy::default()
    };

    let mut running = command(&shell.program, &args, &cwd, false, &[], Some(&policy))
        .await
        .unwrap();
    let poll = running.poll(1_000, 1_024).await.unwrap();

    assert_eq!(poll.exit_code, Some(0), "{}", poll.output);
    assert!(poll.output.contains("dev-null-ok"));
}
