//! Encode and flush bounded newline-delimited mux frames.
use crate::MAX_FRAME_BYTES;
use anyhow::{Context, Result, bail};
use serde::Serialize;
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// Serialize, newline-terminate and flush one bounded JSON frame.
/// # Arguments
/// `writer` is the destination; `value` is the serializable protocol message.
/// # Returns
/// Unit after the frame has been written and flushed.
/// # Errors
/// Reports serialization, frame-size, write and flush failures.
/// # Examples
/// ```no_run
/// # async fn example<W: tokio::io::AsyncWrite + Unpin>(writer: &mut W) {
/// let _result = codetether_mux_protocol::write_frame(writer, &codetether_mux_protocol::ClientRequest::Snapshot).await;
/// # }
/// ```
pub async fn write_frame<T, W>(writer: &mut W, value: &T) -> Result<()>
where
    T: Serialize,
    W: AsyncWrite + Unpin,
{
    let mut bytes = serde_json::to_vec(value).context("encode mux frame")?;
    if bytes.len() > MAX_FRAME_BYTES {
        bail!("mux frame exceeds {MAX_FRAME_BYTES} bytes");
    }
    bytes.push(b'\n');
    writer.write_all(&bytes).await.context("write mux frame")?;
    writer.flush().await.context("flush mux frame")
}
