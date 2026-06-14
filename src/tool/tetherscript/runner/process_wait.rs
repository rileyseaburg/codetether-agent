use std::process::{Child, ExitStatus};
use std::thread;
use std::time::{Duration, Instant};

pub fn wait(mut child: Child, timeout: u64) -> Result<(ExitStatus, bool), String> {
    let started = Instant::now();
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|e| format!("process_run: wait failed: {e}"))?
        {
            return Ok((status, false));
        }
        if started.elapsed() >= Duration::from_millis(timeout) {
            let _ = child.kill();
            return child
                .wait()
                .map(|s| (s, true))
                .map_err(|e| format!("process_run: kill wait failed: {e}"));
        }
        thread::sleep(Duration::from_millis(10));
    }
}
