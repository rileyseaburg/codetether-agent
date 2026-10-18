//! Child process ownership and process creation.
use super::failure::Failure;
use std::process::Stdio;
use tokio::{
    io::BufReader,
    process::{Child, ChildStdin, ChildStdout, Command},
};

pub(super) struct Process {
    pub(super) child: Child,
    pub(super) stdin: ChildStdin,
    pub(super) stdout: BufReader<ChildStdout>,
}

impl Process {
    pub(super) fn spawn() -> Result<Self, Failure> {
        let executable = std::env::current_exe().map_err(|_| Failure::Spawn)?;
        // Use the active executable (including MSIX), not a PATH lookup. Command's
        // defaults preserve the trusted parent's current directory and environment.
        let mut command = Command::new(executable);
        command.args(["windows", "computer-use-worker"]);
        Self::from_command(command)
    }

    pub(super) fn from_command(mut command: Command) -> Result<Self, Failure> {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .map_err(|error| {
                tracing::warn!(os_error = ?error.raw_os_error(), error_kind = ?error.kind(), "Desktop worker spawn failed");
                Failure::Spawn
            })?;
        tracing::info!(pid = ?child.id(), "Desktop worker spawned");
        let stdin = child.stdin.take().ok_or(Failure::Spawn)?;
        let stdout = BufReader::new(child.stdout.take().ok_or(Failure::Spawn)?);
        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        tracing::info!(pid = ?self.child.id(), "Desktop worker dropped; requesting termination");
        let _ = self.child.start_kill();
    }
}
