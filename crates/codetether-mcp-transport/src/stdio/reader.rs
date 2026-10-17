//! Blocking newline-delimited standard-input reader.
use crate::*;
pub(super) fn spawn(read_tx: mpsc::Sender<String>) {
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        for line in reader.lines() {
            match line {
                Ok(msg) if !msg.is_empty() => {
                    trace!(target: "codetether_agent::mcp::transport", "MCP RX: {}", msg);
                    if read_tx.blocking_send(msg).is_err() {
                        break;
                    }
                }
                Ok(_) => continue, // Empty line
                Err(e) => {
                    error!(target: "codetether_agent::mcp::transport", "Failed to read from stdin: {}", e);
                    break;
                }
            }
        }
    });
}
