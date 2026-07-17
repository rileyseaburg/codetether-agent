use super::command;
use crate::tool::bash_shell;

#[tokio::test]
async fn tty_true_allocates_a_real_terminal() {
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push("test -t 0 && test -t 1 && printf tty".into());
    let cwd = std::env::current_dir().unwrap();
    let mut running = command(&shell.program, &args, &cwd, true, &[], None)
        .await
        .unwrap();
    let poll = running.poll(1_000, 1_024).await.unwrap();
    assert!(!poll.running);
    assert_eq!(poll.exit_code, Some(0));
    assert!(poll.output.contains("tty"));
}

#[tokio::test]
async fn terminal_control_c_interrupts_the_foreground_command() {
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push("trap 'printf interrupted; exit 0' INT; printf ready; sleep 30".into());
    let cwd = std::env::current_dir().unwrap();
    let mut running = command(&shell.program, &args, &cwd, true, &[], None)
        .await
        .unwrap();
    let first = running.poll(250, 1_024).await.unwrap();
    assert!(first.running);
    assert!(first.output.contains("ready"));
    running.write("\u{3}").await.unwrap();
    let final_poll = running.poll(2_000, 1_024).await.unwrap();
    assert!(!final_poll.running);
    assert_eq!(final_poll.exit_code, Some(0));
    assert!(final_poll.output.contains("interrupted"));
}
