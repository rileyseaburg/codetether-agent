//! Decode bounded newline-delimited mux frames.
use crate::MAX_FRAME_BYTES;
use anyhow::{Context, Result, bail};
use serde::de::DeserializeOwned;
use tokio::io::{AsyncRead, AsyncReadExt};

/// Read one JSON frame terminated by newline or end-of-stream.
/// # Arguments
/// `reader` supplies framed bytes; `T` is the expected message type.
/// # Returns
/// A decoded message, or None for end-of-stream before any bytes arrive.
/// # Errors
/// Reports read errors, oversized frames and invalid JSON.
/// # Examples
/// ```no_run
/// # async fn example<R: tokio::io::AsyncRead + Unpin>(reader: &mut R) {
/// let _message = codetether_mux_protocol::read_frame::<codetether_mux_protocol::ClientRequest, _>(reader).await;
/// # }
/// ```
pub async fn read_frame<T, R>(reader: &mut R) -> Result<Option<T>>
where
    T: DeserializeOwned,
    R: AsyncRead + Unpin,
{
    let mut bytes = Vec::with_capacity(256);
    let mut byte = [0_u8; 1];
    loop {
        let read = reader.read(&mut byte).await.context("read mux frame")?;
        if read == 0 && bytes.is_empty() {
            return Ok(None);
        }
        if read == 0 || byte[0] == b'\n' {
            break;
        }
        if bytes.len() == MAX_FRAME_BYTES {
            bail!("mux frame exceeds {MAX_FRAME_BYTES} bytes");
        }
        bytes.push(byte[0]);
    }
    serde_json::from_slice(&bytes)
        .context("decode mux frame")
        .map(Some)
}
