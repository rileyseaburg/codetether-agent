use super::{Registry, command};
use crate::tool::bash_shell;

fn shell_args(script: &str) -> (String, Vec<String>) {
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push(script.to_string());
    (shell.program, args)
}

#[tokio::test]
async fn returns_incremental_output_before_process_completion() {
    let (program, args) = shell_args("printf start; sleep 0.5; printf end");
    let cwd = std::env::current_dir().unwrap();
    let mut running = command(&program, &args, &cwd, false, &[], None)
        .await
        .unwrap();
    let first = running.poll(250, 1_024).await.unwrap();
    assert!(first.running);
    assert_eq!(first.output, "start");
    let final_poll = running.poll(1_000, 1_024).await.unwrap();
    assert!(!final_poll.running);
    assert_eq!(final_poll.exit_code, Some(0));
    assert_eq!(final_poll.output, "end");
}

#[tokio::test]
async fn write_stdin_resumes_an_interactive_terminal_session() {
    use crate::tool::{Tool, write_stdin::WriteStdinTool};
    let (program, args) = shell_args("read value; printf 'got:%s' \"$value\"");
    let cwd = std::env::current_dir().unwrap();
    let running = command(&program, &args, &cwd, true, &[], None)
        .await
        .unwrap();
    let registry = std::sync::Arc::new(Registry::default());
    let id = registry.insert(running).await.unwrap();
    let tool = WriteStdinTool::new(registry);
    let result = tool
        .execute(serde_json::json!({"session_id": id, "chars": "hello\n"}))
        .await
        .unwrap();
    assert!(result.success);
    assert!(result.output.contains("got:hello"));
    assert_eq!(result.metadata["running"], serde_json::json!(false));
}
