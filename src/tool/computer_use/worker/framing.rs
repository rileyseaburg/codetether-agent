//! Bounded JSON-lines framing, without unbounded `read_line` allocation.
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

pub(super) const REQUEST_LIMIT: usize = 256 * 1024;
pub(super) const RESPONSE_LIMIT: usize = 12 * 1024 * 1024;

pub(super) async fn read_frame<R: AsyncBufRead + Unpin>(
    reader: &mut R,
    limit: usize,
) -> std::io::Result<Option<Vec<u8>>> {
    let mut frame = Vec::new();
    loop {
        let available = reader.fill_buf().await?;
        if available.is_empty() {
            return if frame.is_empty() {
                Ok(None)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "unterminated worker frame",
                ))
            };
        }
        let newline = available.iter().position(|byte| *byte == b'\n');
        let count = newline.unwrap_or(available.len());
        if count > limit.saturating_sub(frame.len()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "worker frame exceeds size limit",
            ));
        }
        frame.extend_from_slice(&available[..count]);
        reader.consume(count + usize::from(newline.is_some()));
        if newline.is_some() {
            return Ok(Some(frame));
        }
    }
}
