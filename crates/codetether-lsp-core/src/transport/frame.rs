//! Read and validate one framed message from a language server.

use super::*;

pub(super) async fn read(
    reader: &mut BufReader<tokio::process::ChildStdout>,
    header_buf: &mut String,
) -> Option<Vec<u8>> {
    loop {
        let Some(content_length) = headers::read_length(reader, header_buf).await else {
            return None;
        };
        let Some(len) = content_length else {
            warn!("LSP message missing Content-Length header");
            continue;
        };

        if len > MAX_CONTENT_LENGTH {
            error!(
                len,
                cap = MAX_CONTENT_LENGTH,
                "LSP Content-Length exceeds cap; dropping frame to prevent OOM"
            );
            // Drain the oversized body so the stream stays in sync.
            let mut discard = vec![0u8; MAX_CONTENT_LENGTH.min(len)];
            let _ = reader.read_exact(&mut discard).await;
            continue;
        }

        let mut body_buf = vec![0u8; len];
        match reader.read_exact(&mut body_buf).await {
            Ok(_) => return Some(body_buf),
            Err(e) => {
                error!("Failed to read LSP message body: {}", e);
                return None;
            }
        }
    }
}
