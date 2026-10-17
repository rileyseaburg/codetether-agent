//! Start the subprocess and its three I/O tasks in the original order.
use super::*;
impl ProcessTransport {
    /// Spawn a subprocess and connect via stdio.
    ///
    /// # Arguments
    /// * `command` — Executable selected and authorized by the caller.
    /// * `args` — Arguments passed unchanged to that executable.
    /// # Returns
    /// A transport owning the child and channel endpoints.
    /// # Errors
    /// Returns process-spawn errors or errors when a piped stream is unavailable.
    pub async fn spawn(command: &str, args: &[&str]) -> Result<Self> {
        use tokio::process::Command;

        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdout"))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdin"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stderr"))?;

        let (write_tx, write_rx) = mpsc::channel::<String>(100);
        let (read_tx, read_rx) = mpsc::channel::<String>(100);

        // Stderr drain task — capture instead of inheriting so it doesn't corrupt the TUI
        stderr::spawn(stderr);
        writer::spawn(stdin, write_rx);
        reader::spawn(stdout, read_tx);
        Ok(Self {
            _child: child,
            tx: write_tx,
            rx: tokio::sync::Mutex::new(read_rx),
        })
    }
}
