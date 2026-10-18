//! Forward only browser authorization URLs; capture credential JSON privately.

use anyhow::{Result, ensure};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};
#[cfg(test)]
#[path = "stream_tests.rs"]
mod tests;

pub(super) async fn collect(reader: impl AsyncRead + Unpin, json: bool) -> Result<String> {
    let mut lines = BufReader::new(reader.take(1_048_577)).lines();
    let mut output = String::new();
    let mut started = false;
    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if json && trimmed.starts_with('{') {
            started = true;
        }
        if started {
            ensure!(
                output.len() + line.len() < 1_048_576,
                "Vault CLI response exceeded the size limit"
            );
            output.push_str(&line);
            output.push('\n');
        } else if (trimmed.starts_with("https://") || trimmed.starts_with("http://localhost:"))
            && !trimmed.contains('"')
        {
            eprintln!("Open this authorization URL: {trimmed}");
        }
    }
    Ok(output)
}
