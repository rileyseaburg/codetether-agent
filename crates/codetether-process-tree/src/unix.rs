//! Unix process-group ownership for cancellable tool processes.

use std::io;

use tokio::process::Command;

/// Tracks the process group rooted at a configured command.
pub(super) struct Guard {
    pid: Option<i32>,
}

impl Guard {
    /// Records the process-group leader created by `configure`.
    pub(super) fn attach(pid: Option<u32>) -> Self {
        Self {
            pid: pid.and_then(|pid| i32::try_from(pid).ok()),
        }
    }

    /// Forgets a normally completed process group.
    pub(super) fn disarm(&mut self) {
        self.pid = None;
    }

    /// Sends SIGKILL to the complete process group.
    pub(super) fn terminate(&mut self) {
        if let Some(pid) = self.pid.take() {
            // SAFETY: a negative PID targets the process group created below.
            let _ = unsafe { libc::kill(-pid, libc::SIGKILL) };
        }
    }
}

/// Starts the child as a process-group leader for descendant cancellation.
pub(super) fn configure(command: &mut Command) {
    // SAFETY: `pre_exec` calls only the async-signal-safe `setsid` function.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
}
