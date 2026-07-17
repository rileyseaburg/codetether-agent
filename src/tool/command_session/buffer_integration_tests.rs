use super::command;
use crate::tool::bash_shell;

#[tokio::test]
async fn command_poll_retains_output_head_and_tail() {
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push("printf HEAD; printf '%0200d' 0; printf TAIL".to_string());
    let cwd = std::env::current_dir().unwrap();
    let mut running = command(&shell.program, &args, &cwd, false, &[], None)
        .await
        .unwrap();
    let poll = running.poll(1_000, 20).await.unwrap();
    assert!(!poll.running);
    assert!(poll.output.contains("HEAD"));
    assert!(poll.output.contains("TAIL"));
    assert!(poll.output.contains("bytes omitted"));
    assert_eq!(poll.omitted_bytes, 188);
}
