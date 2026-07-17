//! Short grace period for reader tasks after process exit.

use tokio::time::{Duration, Instant, timeout};

use super::super::buffer::Buffer;
use super::super::Running;

pub(super) async fn output(command: &mut Running, output: &mut Buffer) {
    let deadline = Instant::now() + Duration::from_millis(100);
    loop {
        super::drain_ready(command, output);
        if command.output.is_closed() {
            break;
        }
        let now = Instant::now();
        if now >= deadline {
            break;
        }
        if let Ok(Some(chunk)) = timeout(deadline - now, command.output.recv()).await {
            output.push(&chunk);
        }
    }
}
