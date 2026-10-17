//! Newline-delimited writes to subprocess standard input.
use crate::*;
pub(super) fn spawn(mut stdin: tokio::process::ChildStdin, mut write_rx: mpsc::Receiver<String>) {
    tokio::spawn(async move {
        while let Some(msg) = write_rx.recv().await {
            trace!(target: "codetether_agent::mcp::transport", "Process TX: {}", msg);
            if let Err(e) = stdin.write_all(format!("{}\n", msg).as_bytes()).await {
                error!(target: "codetether_agent::mcp::transport", "Failed to write to process stdin: {}", e);
                break;
            }
            if let Err(e) = stdin.flush().await {
                error!(target: "codetether_agent::mcp::transport", "Failed to flush process stdin: {}", e);
                break;
            }
        }
    });
}
