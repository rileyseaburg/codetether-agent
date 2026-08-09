use super::{Registry, command};
use crate::tool::bash_shell;
use crate::tool::{Tool, write_stdin::WriteStdinTool};

fn shell_args(script: &str) -> (String, Vec<String>) {
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push(script.to_string());
    (shell.program, args)
}

#[tokio::test]
async fn silent_poll_reports_the_silence_instead_of_bare_running_heading() {
    let (program, args) = shell_args("sleep 30");
    let cwd = std::env::current_dir().unwrap();
    let running = command(&program, &args, &cwd, false, &[], None)
        .await
        .unwrap();
    let registry = std::sync::Arc::new(Registry::default());
    let id = registry.insert(running).await.unwrap();
    let tool = WriteStdinTool::new(registry);
    let result = tool
        .execute(serde_json::json!({"session_id": id, "yield_time_ms": 5_000}))
        .await
        .unwrap();
    assert!(result.output.contains(&format!("session ID {id}")));
    assert!(result.output.contains("no output for"), "{}", result.output);
    assert_eq!(result.metadata["poll_bytes"], serde_json::json!(0));
    assert_eq!(result.metadata["session_bytes"], serde_json::json!(0));
}

#[tokio::test]
async fn productive_poll_reports_bytes_seen_in_that_poll() {
    let (program, args) = shell_args("printf ready; sleep 30");
    let cwd = std::env::current_dir().unwrap();
    let running = command(&program, &args, &cwd, false, &[], None)
        .await
        .unwrap();
    let registry = std::sync::Arc::new(Registry::default());
    let id = registry.insert(running).await.unwrap();
    let tool = WriteStdinTool::new(registry);
    let result = tool
        .execute(serde_json::json!({"session_id": id, "yield_time_ms": 5_000}))
        .await
        .unwrap();
    assert!(result.output.contains("this poll"), "{}", result.output);
    assert_eq!(result.metadata["poll_bytes"], serde_json::json!(5));
}
