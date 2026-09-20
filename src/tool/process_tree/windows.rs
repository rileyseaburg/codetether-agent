//! Windows Job Object ownership for cancellable tool processes.

use tokio::process::Command;
use windows::Win32::System::JobObjects::TerminateJobObject;
#[path = "windows_job.rs"]
mod job;
#[path = "windows_kill.rs"]
mod kill;
use job::{JobHandle, create_job};
use kill::terminate_with_taskkill;

/// Tracks a child with a Job Object, falling back to `taskkill /T`.
pub(super) struct Guard {
    pid: Option<u32>,
    job: Option<JobHandle>,
}

impl Guard {
    /// Assigns the root process to a private Job Object when possible.
    pub(super) fn attach(pid: Option<u32>) -> Self {
        let job = pid.and_then(|pid| match create_job(pid) {
            Ok(job) => Some(job),
            Err(error) => {
                tracing::warn!(
                    pid,
                    error = %error,
                    "Falling back to taskkill for process-tree cancellation"
                );
                None
            }
        });
        Self { pid, job }
    }

    /// Closes tracking after normal completion without killing descendants.
    pub(super) fn disarm(&mut self) {
        self.job = None;
        self.pid = None;
    }

    /// Terminates every process assigned beneath the command root.
    pub(super) fn terminate(&mut self) {
        if let Some(pid) = self.pid.take() {
            terminate_with_taskkill(pid);
        }
        if let Some(job) = self.job.take() {
            if let Err(error) = unsafe { TerminateJobObject(job.0, 1) } {
                tracing::warn!(
                    error = %error,
                    "Failed to terminate cancelled process Job Object"
                );
            }
        }
    }
}

/// Leaves command creation unchanged until its PID can be assigned.
pub(super) fn configure(_command: &mut Command) {}
