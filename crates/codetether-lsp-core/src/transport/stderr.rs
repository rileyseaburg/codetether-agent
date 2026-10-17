//! stderr responsibility for the stdio LSP transport.

use super::*;

pub(super) fn start(
    stderr: tokio::process::ChildStderr,
    recent_stderr_clone: Arc<RwLock<Vec<String>>>,
    stderr_command: String,
) {
    tokio::spawn(async move {
        let mut reader = BufReader::new(stderr);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => return,
                Ok(_) => {
                    let trimmed = line.trim().to_string();
                    if trimmed.is_empty() {
                        continue;
                    }
                    warn!(command = %stderr_command, stderr = %trimmed, "Language server stderr");
                    let mut guard = recent_stderr_clone.write().await;
                    guard.push(trimmed);
                    if guard.len() > 20 {
                        let excess = guard.len() - 20;
                        guard.drain(0..excess);
                    }
                }
                Err(e) => {
                    warn!(command = %stderr_command, error = %e, "Failed reading language server stderr");
                    return;
                }
            }
        }
    });
}
