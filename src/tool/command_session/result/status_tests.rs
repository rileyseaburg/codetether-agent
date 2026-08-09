use super::suffix;
use crate::tool::command_session::Poll;
use std::time::Duration;

fn poll(running: bool, poll_bytes: usize, silent_secs: u64) -> Poll {
    Poll {
        output: String::new(),
        running,
        exit_code: if running { None } else { Some(0) },
        elapsed: Duration::from_secs(30),
        omitted_bytes: 0,
        poll_bytes,
        silent_for: Duration::from_secs(silent_secs),
        session_bytes: poll_bytes,
    }
}

#[test]
fn silent_running_poll_reports_the_silence_window() {
    let text = suffix(&poll(true, 0, 90));
    assert!(text.contains("no output for 90.0s"), "{text}");
    assert!(text.contains("30.0s elapsed"), "{text}");
}

#[test]
fn productive_running_poll_reports_bytes_for_this_poll() {
    let text = suffix(&poll(true, 2048, 0));
    assert!(text.contains("2.0 KiB this poll"), "{text}");
    assert!(!text.contains("no output"), "{text}");
}

#[test]
fn exited_session_reports_total_output_only() {
    let text = suffix(&poll(false, 12, 0));
    assert!(text.contains("12 B total output"), "{text}");
    assert!(!text.contains("this poll"), "{text}");
}
