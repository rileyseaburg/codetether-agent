//! headers responsibility for the stdio LSP transport.

use super::*;

pub(super) async fn read_length(
    reader: &mut BufReader<tokio::process::ChildStdout>,
    header_buf: &mut String,
) -> Option<Option<usize>> {
    header_buf.clear();
    let mut content_length: Option<usize> = None;

    loop {
        header_buf.clear();
        match reader.read_line(header_buf).await {
            Ok(0) => {
                debug!("LSP server closed connection");
                return None;
            }
            Ok(_) => {
                let line = header_buf.trim();
                if line.is_empty() {
                    break;
                }
                if let Some(stripped) = line.strip_prefix("Content-Length:")
                    && let Ok(len) = stripped.trim().parse::<usize>()
                {
                    content_length = Some(len);
                }
            }
            Err(e) => {
                error!("Failed to read header from LSP server: {}", e);
                return None;
            }
        }
    }

    Some(content_length)
}
