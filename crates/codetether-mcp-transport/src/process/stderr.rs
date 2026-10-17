//! Drain subprocess diagnostics without writing to the terminal.
use crate::*;
pub(super) fn spawn(stderr: tokio::process::ChildStderr) {
    tokio::spawn(async move {
        let mut reader = BufReader::new(stderr);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        warn!(target: "mcp_subprocess", "{trimmed}");
                    }
                }
                Err(_) => break,
            }
        }
    });
}
