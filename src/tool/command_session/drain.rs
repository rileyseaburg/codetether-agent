//! Deadline-bounded collection of incremental process output.

use anyhow::Result;
use tokio::time::{Duration, Instant, timeout};

use super::buffer::Buffer;
use super::{Poll, Running};

#[path = "drain/finish.rs"]
mod finish;

pub(super) async fn poll(command: &mut Running, wait_ms: u64, max: usize) -> Result<Poll> {
    let deadline = Instant::now() + Duration::from_millis(wait_ms);
    let mut output = Buffer::new(max);
    loop {
        drain_ready(command, &mut output);
        if refresh_status(command)? {
            finish::output(command, &mut output).await;
            break;
        }
        let now = Instant::now();
        if now >= deadline {
            break;
        }
        let slice = (deadline - now).min(Duration::from_millis(25));
        if let Ok(Some(chunk)) = timeout(slice, command.output.recv()).await {
            output.push(&chunk);
        }
    }
    let (output, omitted_bytes) = output.finish();
    Ok(Poll {
        output,
        running: command.exit_code.is_none(),
        exit_code: command.exit_code,
        elapsed: command.started.elapsed(),
        omitted_bytes,
    })
}

fn drain_ready(command: &mut Running, output: &mut Buffer) {
    while let Ok(chunk) = command.output.try_recv() {
        output.push(&chunk);
    }
}

fn refresh_status(command: &mut Running) -> std::io::Result<bool> {
    if command.exit_code.is_none()
        && let Some(status) = command.child.try_wait()?
    {
        command.exit_code = Some(status.code().unwrap_or(-1));
        command.stdin = None;
    }
    Ok(command.exit_code.is_some())
}
