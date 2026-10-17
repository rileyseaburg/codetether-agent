//! Parse newline-delimited subprocess standard output into the receive channel.
use crate::*;
pub(super) fn spawn(stdout: tokio::process::ChildStdout, read_tx: mpsc::Sender<String>) {
    tokio::spawn(async move {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        trace!(target: "codetether_agent::mcp::transport", "Process RX: {}", trimmed);
                        if read_tx.send(trimmed.to_string()).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    error!(target: "codetether_agent::mcp::transport", "Failed to read from process stdout: {}", e);
                    break;
                }
            }
        }
    });
}
