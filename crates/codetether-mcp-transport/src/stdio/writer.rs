//! Blocking newline-delimited standard-output writer.
use crate::*;
pub(super) fn spawn(mut write_rx: mpsc::Receiver<String>) {
    std::thread::spawn(move || {
        let mut stdout = std::io::stdout().lock();
        while let Some(msg) = write_rx.blocking_recv() {
            trace!(target: "codetether_agent::mcp::transport", "MCP TX: {}", msg);
            if let Err(e) = writeln!(stdout, "{}", msg) {
                error!(target: "codetether_agent::mcp::transport", "Failed to write to stdout: {}", e);
                break;
            }
            if let Err(e) = stdout.flush() {
                error!(target: "codetether_agent::mcp::transport", "Failed to flush stdout: {}", e);
                break;
            }
        }
    });
}
